use crate::utils::config::{load_config, Config};
use crate::utils::dir::{expand_file_list, media_files};
use crate::utils::metadata::{expected_tags, irrelevant_tags, AurMetadata, AurTags, RawTags};
use crate::utils::tag_validator::TagValidator;
use crate::utils::types::GlobalOpts;
use crate::utils::words::Words;
use colored::Colorize;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
enum CheckResult {
    Good,
    Bad(LintError),
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
enum LintError {
    BomInAlbum,
    BomInArtist,
    BomInGenre,
    BomInTitle,
    EmbeddedArtwork,
    InDiscDirButNoDiscN,
    InvalidAlbum(String),
    InvalidArtist(String),
    InvalidGenre(String),
    InvalidName(String),
    InvalidTitle(String),
    InvalidTNum(u32),
    InvalidYear(i32),
    NotInDiscDirButDiscN,
    UnexpectedTags(Vec<String>),
}

impl LintError {
    pub fn message(&self) -> String {
        match self {
            LintError::BomInAlbum => "BOM found in album tag".to_string(),
            LintError::BomInArtist => "BOM found in artist tag".to_string(),
            LintError::BomInGenre => "BOM found in genre tag".to_string(),
            LintError::BomInTitle => "BOM found in title tag".to_string(),
            LintError::EmbeddedArtwork => "File contains embedded artwork".to_string(),
            LintError::InDiscDirButNoDiscN => {
                "File is in a disc directory but lacks a disc number".to_string()
            }
            LintError::InvalidAlbum(album) => format!("Invalid album tag: {}", album),
            LintError::InvalidArtist(artist) => format!("Invalid artist tag: {}", artist),
            LintError::InvalidGenre(genre) => format!("Invalid genre tag: {}", genre),
            LintError::InvalidName(filename) => format!("Invalid filename: {}", filename),
            LintError::InvalidTitle(title) => format!("Invalid title tag: {}", title),
            LintError::InvalidTNum(tnum) => format!("Invalid track number tag: {}", tnum),
            LintError::InvalidYear(year) => format!("Invalid year tag: {}", year),
            LintError::NotInDiscDirButDiscN => {
                "File has a disc number but is not in a disc directory".to_string()
            }
            LintError::UnexpectedTags(tags) => format!("Unexpected tags: {}", tags.join(", ")),
        }
    }
}

pub fn run(files: &[String], recurse: bool, opts: &GlobalOpts) -> anyhow::Result<()> {
    let config = load_config(&opts.config)?;
    let words = Words::new(&config);
    let validator = TagValidator::new(&words);

    for f in media_files(expand_file_list(files, recurse)?) {
        let results = filter_results(&f, lint_file(&f, &validator)?, &config);
        let problems: Vec<_> = results.iter().filter_map(Some).collect();
        if !problems.is_empty() {
            display_problems(&f, &problems);
        }
    }
    Ok(())
}

fn is_file_excluded(file: &Path, list: Option<&HashSet<String>>) -> bool {
    match list {
        Some(rules) => {
            let file_str = file.to_string_lossy().to_string();
            rules.iter().any(|rule| file_str.contains(rule))
        }
        None => false,
    }
}

fn filter_results(file: &Path, results: Vec<CheckResult>, config: &Config) -> Vec<CheckResult> {
    results
        .into_iter()
        .filter(|r| match r {
            CheckResult::Bad(LintError::InvalidArtist(_)) => {
                !is_file_excluded(file, config.get_ignore_lint_invalid_artist())
            }
            CheckResult::Bad(LintError::InvalidAlbum(_)) => {
                !is_file_excluded(file, config.get_ignore_lint_invalid_album())
            }
            CheckResult::Bad(LintError::InvalidTitle(_)) => {
                !is_file_excluded(file, config.get_ignore_lint_invalid_title())
            }
            _ => true,
        })
        .collect()
}

fn display_problems(file: &Path, problems: &Vec<&CheckResult>) {
    println!("{}", file.display().to_string().bold());
    for p in problems {
        match p {
            CheckResult::Good => (),
            CheckResult::Bad(problem) => println!("  {}", problem.message()),
        }
    }
    println!();
}

fn lint_file(file: &Path, validator: &TagValidator) -> anyhow::Result<Vec<CheckResult>> {
    let info = AurMetadata::new(file)?;
    let results: Vec<_> = run_checks(&info, validator)
        .into_iter()
        .filter(|r| matches!(r, CheckResult::Bad(_)))
        .collect();
    Ok(results)
}

fn run_checks(metadata: &AurMetadata, validator: &TagValidator) -> Vec<CheckResult> {
    vec![
        has_valid_name(&metadata.filename),
        has_no_unwanted_tags(&metadata.filetype, &metadata.rawtags),
        has_no_picture(metadata.has_picture),
        has_no_byte_order_markers(&metadata.tags),
        has_disc_number_or_not(&metadata.path, &metadata.tags.album),
    ]
    .into_iter()
    .chain(has_no_invalid_tags(&metadata.tags, validator))
    .collect()
}

fn has_disc_number_or_not(file: &Path, album_tag: &str) -> CheckResult {
    let disc_in_name = album_tag.contains(" (Disc ");
    let in_disc_dir = file
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .starts_with("disc_");

    if disc_in_name && !in_disc_dir {
        CheckResult::Bad(LintError::NotInDiscDirButDiscN)
    } else if in_disc_dir && !disc_in_name {
        CheckResult::Bad(LintError::InDiscDirButNoDiscN)
    } else {
        CheckResult::Good
    }
}

fn has_no_picture(has_picture: bool) -> CheckResult {
    if has_picture {
        CheckResult::Bad(LintError::EmbeddedArtwork)
    } else {
        CheckResult::Good
    }
}

fn has_valid_name(fname: &str) -> CheckResult {
    let chunks: Vec<_> = fname.split('.').collect();

    if chunks.len() == 4
        && chunks.iter().all(|c| is_safe(c))
        && !chunks[1].starts_with("the_")
        && is_safe_num(chunks[0])
    {
        CheckResult::Good
    } else {
        CheckResult::Bad(LintError::InvalidName(fname.into()))
    }
}

fn has_no_unwanted_tags(filetype: &str, rawtags: &RawTags) -> CheckResult {
    let tag_keys: HashSet<String> = rawtags.iter().map(|(k, _v)| k).cloned().collect();

    let expected_tags = expected_tags(filetype).unwrap();
    let irrelevant_tags = irrelevant_tags(filetype).unwrap();
    let allowed_tags: HashSet<_> = expected_tags.union(&irrelevant_tags).cloned().collect();
    let unexpected_tags: HashSet<_> = tag_keys.difference(&allowed_tags).collect();

    if unexpected_tags.is_empty() {
        CheckResult::Good
    } else {
        let mut tag_vec: Vec<String> = unexpected_tags.iter().map(|s| s.to_string()).collect();
        tag_vec.sort();
        CheckResult::Bad(LintError::UnexpectedTags(tag_vec))
    }
}

fn has_no_invalid_tags(tags: &AurTags, validator: &TagValidator) -> Vec<CheckResult> {
    let mut problems: Vec<CheckResult> = Vec::new();

    if !validator.validate_title(&tags.title) {
        problems.push(CheckResult::Bad(LintError::InvalidTitle(
            tags.title.clone(),
        )));
    }

    if !validator.validate_artist(&tags.artist) {
        problems.push(CheckResult::Bad(LintError::InvalidArtist(
            tags.artist.clone(),
        )));
    }

    if !validator.validate_album(&tags.album) {
        problems.push(CheckResult::Bad(LintError::InvalidAlbum(
            tags.album.clone(),
        )));
    }

    if !validator.validate_t_num(&tags.t_num.to_string()) {
        problems.push(CheckResult::Bad(LintError::InvalidTNum(tags.t_num)));
    }

    if !validator.validate_year(&tags.year.to_string()) {
        problems.push(CheckResult::Bad(LintError::InvalidYear(tags.year)));
    }

    if !validator.validate_genre(&tags.genre) {
        problems.push(CheckResult::Bad(LintError::InvalidGenre(
            tags.genre.clone(),
        )));
    }

    if problems.is_empty() {
        vec![CheckResult::Good]
    } else {
        problems
    }
}

fn is_safe(chunk: &str) -> bool {
    if chunk.starts_with(['-', '_']) || chunk.ends_with(['-', '_']) || chunk.contains("__") {
        return false;
    }

    chunk
        .chars()
        .all(|c| c == '_' || c == '-' || (c.is_alphabetic() && c.is_lowercase()) || c.is_numeric())
}

fn is_safe_num(chunk: &str) -> bool {
    chunk.len() == 2 && chunk != "00" && chunk.chars().all(|c| c.is_numeric())
}

fn has_no_byte_order_markers(tags: &AurTags) -> CheckResult {
    if has_bom_leader(&tags.artist) {
        CheckResult::Bad(LintError::BomInArtist)
    } else if has_bom_leader(&tags.title) {
        CheckResult::Bad(LintError::BomInTitle)
    } else if has_bom_leader(&tags.album) {
        CheckResult::Bad(LintError::BomInAlbum)
    } else if has_bom_leader(&tags.genre) {
        CheckResult::Bad(LintError::BomInGenre)
    } else {
        CheckResult::Good
    }
}

// does string begin with a byte-order marker?
fn has_bom_leader(string: &str) -> bool {
    let bytes = string.as_bytes();
    if bytes.len() < 3 {
        return false;
    }
    bytes[0..=2] == [239, 187, 191]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::{fixture, sample_config};

    #[test]
    fn test_has_valid_name() {
        let valid_names = [
            "01.artist.title.flac",
            "01.artist.title.mp3",
            "03.123.456.flac",
            "05.a_band.a_song-with_brackets.flac",
            "07.some_singer.i-n-i-t-i-a-l-s.flac",
            "19.my_favourite_band.their_best_song.flac",
        ];

        valid_names
            .iter()
            .for_each(|n| assert_eq!(has_valid_name(n), CheckResult::Good));

        let invalid_names = [
            "00.a_band.a_song-with_brackets.flac",
            "01.title.mp3",
            "02.Artist.Title.flac",
            "03._artist.title.mp3",
            "03.artist.title_.mp3",
            "03.artist.title_(with_brackets).flac",
            "03.someone_&_the_somethings.song.mp3",
            "04.too__many.underscores.flac",
            "07.the_somethings.i-n-i-t-i-a-l-s.flac",
            "1.artist.title.flac",
            "19.my_favourite_band.their_best_song!.flac",
            "artist.title.flac",
        ];

        invalid_names.iter().for_each(|n| {
            assert_eq!(
                has_valid_name(n),
                CheckResult::Bad(LintError::InvalidName(n.to_string()))
            )
        });
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe("artist"));
        assert!(is_safe("01"));
        assert!(is_safe("a"));
        assert!(is_safe("7"));
        assert!(is_safe("me"));
        assert!(is_safe("two_words"));
        assert!(is_safe("and_three_words"));
        assert!(is_safe("some--bracketed--words"));
        assert!(is_safe("with-hyphen"));
        assert!(is_safe("1_two_3"));
        assert!(is_safe("one_2_3"));

        assert!(!is_safe("_word"));
        assert!(!is_safe("-word"));
        assert!(!is_safe("_"));
        assert!(!is_safe("-"));
        assert!(!is_safe("word_"));
        assert!(!is_safe("two__words"));
        assert!(!is_safe("tres,comma"));
        assert!(!is_safe("Word"));
    }

    #[test]
    fn test_is_safe_num() {
        assert!(is_safe_num("01"));
        assert!(is_safe_num("99"));
        assert!(!is_safe_num("00"));
    }

    #[test]
    fn test_allow_from_config() {
        let words = Words::new(&sample_config());
        let validator = TagValidator::new(&words);
        let config = sample_config();
        let file = fixture("commands/lint/09.tester.bad_title_allowed.mp3");
        let lint_result = lint_file(&file, &validator).unwrap();
        let expected_empty: Vec<CheckResult> = Vec::new();

        assert_eq!(expected_empty, filter_results(&file, lint_result, &config));
    }

    #[test]
    fn lint_functional_tests() {
        let words = Words::new(&sample_config());
        let validator = TagValidator::new(&words);

        assert!(lint_file(
            &fixture("commands/lint/01.tester.lints_fine.flac"),
            &validator
        )
        .unwrap()
        .is_empty());

        assert!(lint_file(
            &fixture("commands/lint/02.tester.lints_fine.mp3"),
            &validator
        )
        .unwrap()
        .is_empty());

        assert_eq!(
            vec![
                CheckResult::Bad(LintError::InvalidName(
                    "00.tester.missing_genre_track_no_year.flac".into()
                )),
                CheckResult::Bad(LintError::InvalidTNum(0)),
                CheckResult::Bad(LintError::InvalidYear(0)),
                CheckResult::Bad(LintError::InvalidGenre("".into())),
            ],
            lint_file(
                &fixture("commands/lint/00.tester.missing_genre_track_no_year.flac"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![CheckResult::Bad(LintError::BomInTitle)],
            lint_file(
                &fixture("commands/lint/03.tester.has_bom_leader.flac"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![CheckResult::Bad(LintError::UnexpectedTags(vec![
                "tdrc".into(),
                "txxx".into(),
            ]))],
            lint_file(
                &fixture("commands/lint/05.tester.surplus_tags.mp3"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![
                CheckResult::Bad(LintError::UnexpectedTags(vec![
                    "apic".into(),
                    "tcom".into(),
                    "tenc".into(),
                    "txxx".into(),
                ])),
                CheckResult::Bad(LintError::EmbeddedArtwork)
            ],
            lint_file(
                &fixture("commands/lint/06.tester.extra_tags_and_picture.mp3"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![CheckResult::Bad(LintError::EmbeddedArtwork)],
            lint_file(&fixture("commands/lint/07.tester.picture.flac"), &validator).unwrap()
        );

        assert_eq!(
            vec![CheckResult::Bad(LintError::InDiscDirButNoDiscN)],
            lint_file(
                &fixture("commands/lint/disc_1/01.tester.no_disc_number.mp3"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![CheckResult::Bad(LintError::NotInDiscDirButDiscN)],
            lint_file(
                &fixture("commands/lint/08.tester.disc_number.mp3"),
                &validator
            )
            .unwrap()
        );
    }
}
