use crate::utils::dir::{expand_file_list, media_files};
use crate::utils::metadata::AurMetadata;
use crate::utils::string::Compacted;
use crate::utils::types::GlobalOpts;
use anyhow::anyhow;
use indicatif::ProgressBar;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

type ArtistDirs = HashMap<String, HashSet<PathBuf>>;
type Dupes = Vec<DupeCluster>;
type DupeCluster = HashMap<String, HashSet<PathBuf>>;

pub fn run(root_dir: &str, opts: &GlobalOpts) -> anyhow::Result<()> {
    for cluster in find_dupes(root_dir.to_string(), opts)? {
        println!("{}", format_dupes(&cluster));
    }

    Ok(())
}

fn find_dupes(root_dir: String, opts: &GlobalOpts) -> anyhow::Result<Dupes> {
    let all_files = media_files(expand_file_list(&[root_dir], true)?);

    if all_files.is_empty() {
        return Err(anyhow!("No files found"));
    }

    let unique_artists = artist_dirs(all_files, opts)?;
    let mut ret: Dupes = check_thes(&unique_artists);
    ret.extend(check_compacted(&unique_artists));

    Ok(ret)
}

fn artist_dirs(file_hash: HashSet<PathBuf>, opts: &GlobalOpts) -> anyhow::Result<ArtistDirs> {
    let mut ret: ArtistDirs = HashMap::new();

    let bar = if opts.verbose {
        Some(ProgressBar::new(file_hash.len() as u64))
    } else {
        None
    };

    for file in file_hash {
        let info = AurMetadata::new(&file)?;
        if let Some(ref bar) = bar {
            bar.inc(1);
        }
        let dir = file.parent().unwrap();
        ret.entry(info.tags.artist)
            .or_default()
            .insert(dir.to_owned());
    }

    if let Some(ref bar) = bar {
        bar.finish();
    }

    Ok(ret)
}

fn check_thes(artists: &ArtistDirs) -> Dupes {
    let thes = artists.keys().filter(|k| k.starts_with("The "));

    let mut ret: Dupes = Vec::new();

    for the in thes {
        let no_the = the.replacen("The ", "", 1);
        if artists.contains_key(&no_the) {
            ret.push(HashMap::from([
                (the.to_owned(), artists.get(the).unwrap().to_owned()),
                (no_the.to_owned(), artists.get(&no_the).unwrap().to_owned()),
            ]));
        }
    }

    ret
}

fn check_compacted(artists: &ArtistDirs) -> Dupes {
    let mut groups: HashMap<String, DupeCluster> = HashMap::new();

    for (artist, dirs) in artists {
        let compacted = artist.compacted();
        let dc: DupeCluster = HashMap::from([(artist.to_string(), dirs.to_owned())]);

        groups
            .entry(compacted)
            .and_modify(|e| {
                for (key, value) in &dc {
                    e.entry(key.clone())
                        .or_insert_with(HashSet::new)
                        .extend(value.to_owned());
                }
            })
            .or_insert(dc);
    }

    groups
        .into_iter()
        .filter(|(_, cluster)| cluster.len() > 1)
        .map(|(_, cluster)| cluster)
        .collect()
}

fn format_dupes(dupe_cluster: &DupeCluster) -> String {
    let mut ret = String::new();

    for (name, paths) in dupe_cluster {
        ret.push_str(name.as_str());
        for path in paths {
            ret.push_str(format!("\n    {}", path.display()).as_str());
        }
        ret.push('\n');
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::defopts;
    use assert_unordered::assert_eq_unordered;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};

    #[test]
    fn test_artist_list_flac() {
        let fixture_dir = fixture_as_string("commands/namecheck/flac");
        let all_files = expand_file_list(&[fixture_dir], true).unwrap();

        assert_eq_unordered!(
            flac_artist_list(),
            artist_dirs(all_files, &defopts()).unwrap()
        );
    }

    #[test]
    fn test_artist_list_mp3() {
        let fixture_dir = fixture_as_string("commands/namecheck/mp3");
        let all_files = expand_file_list(&[fixture_dir], true).unwrap();

        assert_eq_unordered!(
            mp3_artist_list(),
            artist_dirs(all_files, &defopts()).unwrap()
        );
    }

    #[test]
    fn test_check_thes_matches_flac() {
        let mut expected: Dupes = Vec::new();
        let mut expected_cluster: DupeCluster = HashMap::new();

        expected_cluster.insert(
            "Artist".to_string(),
            HashSet::from([fixture("commands/namecheck/flac/thes/tracks")]),
        );
        expected_cluster.insert(
            "The Artist".to_string(),
            HashSet::from([fixture(
                "commands/namecheck/flac/thes/albums/abc/artist.album",
            )]),
        );

        expected.push(expected_cluster);

        assert_eq_unordered!(expected, check_thes(&flac_artist_list()));
    }

    #[test]
    fn test_check_thes_matches_mp3() {
        assert!(check_thes(&mp3_artist_list()).is_empty());
    }

    #[test]
    fn test_check_compacted() {
        let mut expected: Dupes = Vec::new();
        let mut expected_cluster: DupeCluster = HashMap::new();

        expected_cluster.insert(
            "The B52s".to_string(),
            HashSet::from([fixture("commands/namecheck/mp3/similar/tracks")]),
        );

        expected_cluster.insert(
            "The B-52's".to_string(),
            HashSet::from([fixture(
                "commands/namecheck/mp3/similar/albums/b-52s.wild_planet",
            )]),
        );

        expected_cluster.insert(
            "The B52's".to_string(),
            HashSet::from([fixture("commands/namecheck/mp3/similar/tracks")]),
        );

        expected.push(expected_cluster);

        assert_eq_unordered!(&expected, &check_compacted(&mp3_artist_list()));
    }

    // Views of the resource directories, used as test inputs and outputs
    fn flac_artist_list() -> ArtistDirs {
        HashMap::from([
            (
                "Artist".to_string(),
                HashSet::from([fixture("commands/namecheck/flac/thes/tracks")]),
            ),
            (
                "The Artist".to_string(),
                HashSet::from([fixture(
                    "commands/namecheck/flac/thes/albums/abc/artist.album",
                )]),
            ),
            (
                "Singer".to_string(),
                HashSet::from([fixture("commands/namecheck/flac/thes/tracks")]),
            ),
        ])
    }

    fn mp3_artist_list() -> ArtistDirs {
        HashMap::from([
            (
                "The B52s".to_string(),
                HashSet::from([fixture("commands/namecheck/mp3/similar/tracks")]),
            ),
            (
                "The B-52's".to_string(),
                HashSet::from([fixture(
                    "commands/namecheck/mp3/similar/albums/b-52s.wild_planet",
                )]),
            ),
            (
                "The B52's".to_string(),
                HashSet::from([fixture("commands/namecheck/mp3/similar/tracks")]),
            ),
        ])
    }
}
