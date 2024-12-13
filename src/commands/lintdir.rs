use crate::utils::config::{load_config, Config, MAX_ARTWORK_SIZE, MIN_ARTWORK_SIZE};
use crate::utils::dir::{expand_dir_list, media_files};
use crate::utils::metadata::AurMetadata;
use crate::utils::rename::number_from_filename;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use anyhow::anyhow;
use colored::Colorize;
use image::GenericImageView;
use image::ImageReader;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::{self, canonicalize};
use std::path::{Path, PathBuf};

const ARTIST_SEPS: [&str; 6] = ["feat", "feat.", "featuring", "and", "with", "/"];

lazy_static! {
    static ref DIRNAME_REGEX: Regex =
        Regex::new(r"^[a-z0-9][a-z\-_0-9]+\.[a-z0-9][a-z\-_[0-9]]*[a-z0-9]?$").unwrap();
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
enum CheckResult {
    Good,
    Bad(LintDirError),
}

#[derive(PartialEq)]
enum Hierarchy {
    Flac,
    Mp3,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
enum LintDirError {
    BadFile(HashSet<String>),
    BadFileCount,
    CoverArtInvalid(String),
    CoverArtNotSquare,
    CoverArtTooBig,
    CoverArtTooSmall,
    InconsistentTags(HashSet<String>),
    InvalidDirName,
    MixedFileTypes,
    UnsequencedFile,
}

impl LintDirError {
    pub fn message(&self) -> String {
        match self {
            LintDirError::BadFile(files) => {
                let mut vec: Vec<_> = files.iter().map(|f| f.to_string()).collect();
                vec.sort();
                format!("Unexpected file(s): {}", vec.join(", "))
            }
            LintDirError::BadFileCount => "Unexpected number of files".to_string(),
            LintDirError::CoverArtInvalid(err) => format!("Could not validate cover art: {}", err),
            LintDirError::CoverArtNotSquare => "Cover art is not square".to_string(),
            LintDirError::CoverArtTooBig => "Cover art is too big".to_string(),
            LintDirError::CoverArtTooSmall => "Cover art is too small".to_string(),
            LintDirError::InconsistentTags(tags) => {
                let mut vec: Vec<_> = tags.iter().map(|f| f.to_string()).collect();
                vec.sort();
                format!("Inconsistent tags: {}", vec.join(", "))
            }
            LintDirError::InvalidDirName => "Invalid directory name".to_string(),
            LintDirError::MixedFileTypes => "Mixed file types".to_string(),
            LintDirError::UnsequencedFile => "File numbers are not correctly sequenced".to_string(),
        }
    }
}

pub fn run(dirlist: &[String], recurse: bool, opts: &GlobalOpts) -> anyhow::Result<()> {
    let config = load_config(&opts.config)?;
    let dirs_to_list: Vec<PathBuf> = dirlist.iter().map(PathBuf::from).collect();
    // let mut found_problems = false;

    for dir in expand_dir_list(&dirs_to_list, recurse) {
        let dir = canonicalize(dir)?;
        if let Some(res) = lint_dir(&dir, opts)? {
            let results = filter_results(&dir, res, &config);
            let problems: Vec<_> = results.iter().filter_map(Some).collect();

            if !problems.is_empty() {
                display_problems(&dir, &problems);
                // found_problems = true;
            }
        }
    }

    // Ok(!found_problems)
    Ok(())
}

fn display_problems(dir: &Path, problems: &Vec<&CheckResult>) {
    println!("{}", dir.display().to_string().bold());
    for p in problems {
        match p {
            CheckResult::Good => (),
            CheckResult::Bad(problem) => println!("  {}", problem.message()),
        }
    }
    println!();
}

fn is_dir_excluded(file: &Path, list: Option<&HashSet<String>>) -> bool {
    match list {
        Some(rules) => {
            let file_str = file.to_string_lossy().to_string();
            rules.iter().any(|rule| file_str.contains(rule))
        }
        None => false,
    }
}

fn filter_results(dir: &Path, results: Vec<CheckResult>, config: &Config) -> Vec<CheckResult> {
    results
        .into_iter()
        .filter(|r| match r {
            CheckResult::Bad(LintDirError::BadFileCount) => {
                !is_dir_excluded(dir, config.get_ignore_lintdir_bad_file_count())
            }
            CheckResult::Bad(LintDirError::InconsistentTags(_)) => {
                !is_dir_excluded(dir, config.get_ignore_lintdir_inconsistent_tags())
            }
            _ => true,
        })
        .collect()
}

fn lint_dir(dir: &Path, opts: &GlobalOpts) -> anyhow::Result<Option<Vec<CheckResult>>> {
    let all_files = files_in_dir(dir)?;
    let all_metadata = metadata_for(&all_files)?;

    if all_metadata.is_empty() {
        return Ok(None);
    }

    if dir.file_name().unwrap().to_string_lossy() == "tracks" {
        return Ok(None);
    }

    verbose!(opts, "Linting {}", dir.display());

    let hierarchy = if dir
        .components()
        .any(|c| c.as_os_str() == OsStr::new("flac"))
    {
        Hierarchy::Flac
    } else if dir.components().any(|c| c.as_os_str() == OsStr::new("mp3")) {
        Hierarchy::Mp3
    } else {
        return Err(anyhow!(
            "unable to determine media hierarchy from {}",
            dir.display()
        ));
    };

    let results: Vec<_> = run_checks(dir, &all_files, &all_metadata, hierarchy)
        .into_iter()
        .filter(|r| matches!(r, CheckResult::Bad(_)))
        .collect();

    Ok(Some(results))
}

fn run_checks(
    dir: &Path,
    all_files: &HashSet<PathBuf>,
    all_metadata: &[AurMetadata],
    hierarchy: Hierarchy,
) -> Vec<CheckResult> {
    let mut checks = vec![
        is_correctly_named(dir, false),
        has_no_bad_files(dir, all_files, &hierarchy),
        has_right_file_count(all_files),
        has_consistent_tags(dir, all_metadata),
        all_files_are_same_type(all_metadata),
    ];

    if hierarchy == Hierarchy::Flac {
        checks.push(has_suitable_cover_art(dir));
    }

    checks
}

// An album directory should be of the form 'artist_name.album_name', but can have sub-directories.
// So, if we find content in an incorrectly named directory, we examine the parent,
fn is_correctly_named(dir: &Path, on_retry: bool) -> CheckResult {
    let name = dir.file_name().unwrap().to_string_lossy();

    if name == "tracks" {
        return CheckResult::Good;
    }

    if DIRNAME_REGEX.is_match(&name) && !name.starts_with("the_") {
        return CheckResult::Good;
    }

    if on_retry {
        CheckResult::Bad(LintDirError::InvalidDirName)
    } else {
        is_correctly_named(dir.parent().unwrap(), true)
    }
}

fn has_no_bad_files(
    dir: &Path,
    file_list: &HashSet<PathBuf>,
    hierarchy: &Hierarchy,
) -> CheckResult {
    let media = media_files(file_list);
    let mut non_media: HashSet<PathBuf> = file_list
        .difference(&media)
        .map(|f| f.to_path_buf())
        .collect();

    if hierarchy == &Hierarchy::Flac {
        let artwork = dir.join("front.jpg");
        non_media.remove(&artwork);
    }

    if non_media.is_empty() {
        CheckResult::Good
    } else {
        CheckResult::Bad(LintDirError::BadFile(
            non_media
                .iter()
                .map(|f| f.to_string_lossy().to_string())
                .collect(),
        ))
    }
}

fn has_right_file_count(file_list: &HashSet<PathBuf>) -> CheckResult {
    let media = media_files(file_list);
    let file_nums: Vec<u32> = media
        .iter()
        .filter_map(|f| {
            number_from_filename(&f.file_name().unwrap().to_string_lossy()).map(|(_, num)| num)
        })
        .collect();

    if file_nums.iter().max() != Some(&(media.len() as u32)) {
        return CheckResult::Bad(LintDirError::BadFileCount);
    }

    for i in 1..=media.len() {
        if !file_nums.contains(&(i as u32)) {
            return CheckResult::Bad(LintDirError::UnsequencedFile);
        }
    }

    CheckResult::Good
}

fn looks_like_compilation(dir: &Path, on_retry: bool) -> bool {
    let dirname = dir
        .file_name()
        .expect("looks_like_compilation couldn't parse dir")
        .to_string_lossy()
        .to_string();

    let bits: Vec<&str> = dirname.split('.').collect();

    if bits.len() == 1 && !on_retry {
        // probably a disc dir
        looks_like_compilation(dir.parent().unwrap(), true)
    } else {
        bits[0] == "various" || bits[0].contains("--")
    }
}

fn inconsistencies_are_featuring(metadata: &[AurMetadata]) -> bool {
    let primaries: Vec<String> = metadata
        .iter()
        .map(|m| {
            let mut shortest = m.tags.artist.clone();

            for sep in ARTIST_SEPS {
                let sep_str = format!(" {} ", sep);
                let bits: Vec<_> = m.tags.artist.split(&sep_str).collect();
                let first_bit = bits[0].trim();
                if first_bit.len() < shortest.len() {
                    shortest = first_bit.to_string();
                }
            }

            shortest
        })
        .collect();

    let ref_artist = &primaries[0].to_string();
    primaries.iter().all(|m| m == ref_artist)
}

fn has_consistent_tags(dir: &Path, metadata: &[AurMetadata]) -> CheckResult {
    let mut inconsistent_tags: HashSet<String> = HashSet::new();

    if !metadata
        .iter()
        .all(|m| m.tags.artist == metadata[0].tags.album)
        && !looks_like_compilation(dir, false)
        && !inconsistencies_are_featuring(metadata)
    {
        inconsistent_tags.insert("artist".into());
    }

    if !metadata
        .iter()
        .all(|m| m.tags.album == metadata[0].tags.album)
    {
        inconsistent_tags.insert("album".into());
    }

    if !metadata
        .iter()
        .all(|m| m.tags.year == metadata[0].tags.year)
    {
        inconsistent_tags.insert("year".into());
    }

    if !metadata
        .iter()
        .all(|m| m.tags.genre == metadata[0].tags.genre)
    {
        inconsistent_tags.insert("genre".into());
    }

    if inconsistent_tags.is_empty() {
        CheckResult::Good
    } else {
        CheckResult::Bad(LintDirError::InconsistentTags(inconsistent_tags))
    }
}

fn has_suitable_cover_art(dir: &Path) -> CheckResult {
    let artwork = dir.join("front.jpg");

    let img = match ImageReader::open(artwork) {
        Ok(handle) => match handle.decode() {
            Ok(data) => data,
            Err(e) => return CheckResult::Bad(LintDirError::CoverArtInvalid(e.to_string())),
        },
        Err(e) => return CheckResult::Bad(LintDirError::CoverArtInvalid(e.to_string())),
    };

    let (x, y) = img.dimensions();

    if x != y {
        CheckResult::Bad(LintDirError::CoverArtNotSquare)
    } else if x > MAX_ARTWORK_SIZE {
        CheckResult::Bad(LintDirError::CoverArtTooBig)
    } else if x < MIN_ARTWORK_SIZE {
        CheckResult::Bad(LintDirError::CoverArtTooSmall)
    } else {
        CheckResult::Good
    }
}

fn all_files_are_same_type(metadata: &[AurMetadata]) -> CheckResult {
    let filetypes: HashSet<&str> = metadata.iter().map(|m| m.filetype.as_str()).collect();
    if filetypes.len() == 1 {
        CheckResult::Good
    } else {
        CheckResult::Bad(LintDirError::MixedFileTypes)
    }
}

fn files_in_dir(dir: &Path) -> anyhow::Result<HashSet<PathBuf>> {
    let entries = fs::read_dir(dir)?;
    Ok(entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect())
}

fn metadata_for(files: &HashSet<PathBuf>) -> anyhow::Result<Vec<AurMetadata>> {
    let mut ret = Vec::new();

    for f in media_files(files).iter() {
        ret.push(AurMetadata::new(f)?);
    }

    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_all_files_are_same_type() {
        assert_eq!(
            CheckResult::Good,
            all_files_are_same_type(&metadata_for(&perfect_flac()).unwrap())
        );
        assert_eq!(
            CheckResult::Good,
            all_files_are_same_type(&metadata_for(&perfect_mp3()).unwrap())
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::MixedFileTypes),
            all_files_are_same_type(
                &metadata_for(
                    &files_in_dir(&fixture("commands/lintdir/mp3/tester.mixed_types")).unwrap()
                )
                .unwrap()
            )
        );
    }

    #[test]
    fn test_has_suitable_cover_art() {
        assert_eq!(
            CheckResult::Good,
            has_suitable_cover_art(&perfect_flac_dir())
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::CoverArtInvalid(
                "No such file or directory (os error 2)".into()
            )),
            has_suitable_cover_art(&fixture("commands/lintdir/flac/tester.artwork_missing"))
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::CoverArtTooBig),
            has_suitable_cover_art(&fixture("commands/lintdir/flac/tester.artwork_too_big"))
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::CoverArtTooSmall),
            has_suitable_cover_art(&fixture("commands/lintdir/flac/tester.artwork_too_small"))
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::CoverArtNotSquare),
            has_suitable_cover_art(&fixture("commands/lintdir/flac/tester.artwork_not_square"))
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::CoverArtInvalid(
                "Format error decoding Jpeg: \"No more bytes\"".into()
            )),
            has_suitable_cover_art(&fixture("commands/lintdir/flac/tester.artwork_invalid"))
        );
    }

    #[test]
    fn test_has_consistent_tags() {
        assert_eq!(
            CheckResult::Good,
            has_consistent_tags(&perfect_flac_dir(), &metadata_for(&perfect_flac()).unwrap())
        );

        assert_eq!(
            CheckResult::Good,
            has_consistent_tags(&perfect_mp3_dir(), &metadata_for(&perfect_mp3()).unwrap())
        );

        assert_eq!(
            CheckResult::Good,
            has_consistent_tags(
                &fixture("commands/lintdir/mp3/tester.perfect--featuring"),
                &metadata_for(
                    &files_in_dir(&fixture("commands/lintdir/mp3/tester.perfect--featuring"))
                        .unwrap()
                )
                .unwrap()
            )
        );

        assert_eq!(
            CheckResult::Good,
            has_consistent_tags(
                &fixture("commands/lintdir/mp3/artist--band.split_single"),
                &metadata_for(
                    &files_in_dir(&fixture("commands/lintdir/mp3/artist--band.split_single"))
                        .unwrap()
                )
                .unwrap()
            )
        );

        assert_eq!(
            CheckResult::Good,
            has_consistent_tags(
                &fixture("commands/lintdir/mp3/various.compilation"),
                &metadata_for(
                    &files_in_dir(&fixture("commands/lintdir/mp3/various.compilation")).unwrap()
                )
                .unwrap()
            )
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::InconsistentTags(HashSet::from([
                "album".into()
            ]))),
            has_consistent_tags(
                &fixture("commands/lintdir/flac/tester.different_album"),
                &metadata_for(
                    &files_in_dir(&fixture("commands/lintdir/flac/tester.different_album"))
                        .unwrap()
                )
                .unwrap()
            )
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::InconsistentTags(HashSet::from([
                "album".into(),
                "genre".into(),
                "year".into()
            ]))),
            has_consistent_tags(
                &fixture("commands/lintdir/mp3/tester.mixed_genre_year_album"),
                &metadata_for(
                    &files_in_dir(&fixture(
                        "commands/lintdir/mp3/tester.mixed_genre_year_album"
                    ))
                    .unwrap()
                )
                .unwrap()
            )
        );
    }

    #[test]
    fn test_has_right_file_count() {
        assert_eq!(CheckResult::Good, has_right_file_count(&perfect_flac()));
        assert_eq!(CheckResult::Good, has_right_file_count(&perfect_mp3()));

        assert_eq!(
            CheckResult::Bad(LintDirError::UnsequencedFile),
            has_right_file_count(
                &files_in_dir(&fixture("commands/lintdir/mp3/tester.wrongly_numbered")).unwrap()
            )
        );

        assert_eq!(
            CheckResult::Bad(LintDirError::BadFileCount),
            has_right_file_count(
                &files_in_dir(&fixture("commands/lintdir/flac/tester.missing_file")).unwrap()
            )
        );
    }

    #[test]
    fn test_is_correctly_named() {
        let good_names = [
            "eps/band.ep",
            "albums/abc/artist.classic_album--remaster",
            "albums/abc/artist.classic_album--remaster/bonus_disc",
            "albums/abc/cass_mccombs.a",
            "albums/pqrs/singer.double_album/disc_1",
            "albums/pqrs/singer.double_album/disc_2",
        ];

        let bad_names = [
            "eps/self-titled",
            "eps/fatima_mansions.1000%",
            "eps/!!!.ep",
            "albums/pqrs/Slint.Spiderland",
            "albums/pqrs/smiths.the.smiths",
            "albums/pqrs/the_smiths.the_smiths",
        ];

        good_names.iter().for_each(|name| {
            assert_eq!(
                CheckResult::Good,
                is_correctly_named(&PathBuf::from(name), false),
                "{} is bad",
                name,
            )
        });

        bad_names.iter().for_each(|name| {
            assert_eq!(
                CheckResult::Bad(LintDirError::InvalidDirName),
                is_correctly_named(&PathBuf::from(name), false),
                "{} is bad",
                name,
            )
        });
    }

    #[test]
    fn test_has_no_bad_files() {
        assert_eq!(
            CheckResult::Good,
            has_no_bad_files(
                &fixture("commands/lintdir/flac/tester.perfect"),
                &perfect_flac(),
                &Hierarchy::Flac
            )
        );

        assert_eq!(
            CheckResult::Good,
            has_no_bad_files(
                &fixture("commands/lintdir/mp3/tester.perfect"),
                &perfect_mp3(),
                &Hierarchy::Mp3
            )
        );

        let junk_file_dir = fixture("commands/lintdir/flac/tester.junk_files");
        let junk_files = HashSet::from([
            junk_file_dir.join("rip.log").to_string_lossy().to_string(),
            junk_file_dir
                .join("Back-Cover.jpg")
                .to_string_lossy()
                .to_string(),
        ]);

        assert_eq!(
            CheckResult::Bad(LintDirError::BadFile(junk_files)),
            has_no_bad_files(
                &junk_file_dir,
                &files_in_dir(&junk_file_dir).unwrap(),
                &Hierarchy::Flac
            )
        );

        let unwanted_art_dir = fixture("commands/lintdir/mp3/tester.unwanted_art");
        let unwanted_art_files = HashSet::from([unwanted_art_dir
            .join("front.jpg")
            .to_string_lossy()
            .to_string()]);

        assert_eq!(
            CheckResult::Bad(LintDirError::BadFile(unwanted_art_files)),
            has_no_bad_files(
                &unwanted_art_dir,
                &files_in_dir(&unwanted_art_dir).unwrap(),
                &Hierarchy::Mp3
            )
        );
    }

    fn perfect_flac_dir() -> PathBuf {
        fixture("commands/lintdir/flac/tester.perfect")
    }

    fn perfect_flac() -> HashSet<PathBuf> {
        files_in_dir(&perfect_flac_dir()).unwrap()
    }

    fn perfect_mp3_dir() -> PathBuf {
        fixture("commands/lintdir/mp3/tester.perfect")
    }

    fn perfect_mp3() -> HashSet<PathBuf> {
        files_in_dir(&perfect_mp3_dir()).unwrap()
    }
}
