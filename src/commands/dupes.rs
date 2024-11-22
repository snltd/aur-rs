use crate::utils::dir;
use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

type Dupes = Vec<Vec<PathBuf>>;

lazy_static! {
    static ref NO_LEADING_NUMBER: Regex = Regex::new(r"^\d\d\.(.*)$").unwrap();
}

pub fn run(root_dir: &str) -> anyhow::Result<()> {
    let dupes = dupes_under(&PathBuf::from(root_dir))?;
    dupes.iter().for_each(|d| println!("{}", format_dupes(d)));
    Ok(())
}

fn format_dupes(dupe_cluster: &[PathBuf]) -> String {
    let mut ret: String = format!("{}", dupe_cluster.first().unwrap().display());
    dupe_cluster[1..]
        .iter()
        .for_each(|d| ret.push_str(&format!("\n  {}", d.display())));
    ret.push('\n');

    ret
}

fn file_hash(paths: &HashSet<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    let mut ret: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for p in paths {
        if let Some(fname) = filename_from_file(p) {
            ret.entry(fname).or_default().push(p.clone());
        }
    }

    ret
}

fn dupes_under(dir: &Path) -> anyhow::Result<Dupes> {
    for d in ["tracks", "eps", "albums"] {
        let required_dir = dir.join(d);
        if !required_dir.exists() {
            return Err(anyhow!(format!("{} not found", required_dir.display())));
        }
    }

    let needle_files = dir::expand_file_list(
        &[dir.join("tracks")].map(|d| d.to_string_lossy().to_string()),
        true,
    )?;

    let haystack_files = dir::expand_file_list(
        &[dir.join("albums"), dir.join("eps")].map(|d| d.to_string_lossy().to_string()),
        true,
    )?;

    let needle_hash = file_hash(&needle_files);
    let haystack_hash = file_hash(&haystack_files);

    let mut ret: Dupes = Vec::new();

    for (name, mut paths) in needle_hash {
        if let Some(m) = haystack_hash.get(&name) {
            let mut n = m.clone();
            n.sort();
            paths.extend_from_slice(&n);
            ret.push(paths);
        }
    }

    Ok(ret)
}

fn filename_from_file(path: &Path) -> Option<String> {
    if let Some(name) = path.file_name() {
        if let Some(name_str) = name.to_str() {
            if let Some(c) = NO_LEADING_NUMBER.captures(name_str) {
                return Some(c[1].to_string());
            }
            return Some(name_str.to_string());
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    fn test_missing_arg() {
        assert!(dupes_under(&PathBuf::from("/does/not/exist")).is_err());
    }

    #[test]
    fn test_filename_from_file() {
        assert_eq!(
            "singer.song.flac".to_string(),
            filename_from_file(&PathBuf::from("/path/to/singer.album/02.singer.song.flac"))
                .unwrap()
        );

        assert_eq!(
            "singer.song.flac".to_string(),
            filename_from_file(&PathBuf::from("/path/to/singer.album/singer.song.flac")).unwrap()
        );
    }

    #[test]
    fn test_file_hash() {
        let mut input = HashSet::new();
        input.insert(PathBuf::from("/flac/eps/singer.ep/01.singer.song_1.flac"));
        input.insert(PathBuf::from("/flac/eps/singer.ep/02.singer.song_2.flac"));
        input.insert(PathBuf::from(
            "/flac/albums/pqrs/singer.lp/06.singer.song_1.flac",
        ));
        input.insert(PathBuf::from("/flac/eps/singer.ep/front.jpg"));

        let result = file_hash(&input);

        assert_eq!(3, result.len());
        assert_eq!(
            &vec![PathBuf::from("/flac/eps/singer.ep/02.singer.song_2.flac")],
            result.get("singer.song_2.flac").unwrap()
        );

        let expected = &vec![
            PathBuf::from("/flac/albums/pqrs/singer.lp/06.singer.song_1.flac"),
            PathBuf::from("/flac/eps/singer.ep/01.singer.song_1.flac"),
        ];

        let mut sorted_result = result.get("singer.song_1.flac").unwrap().clone();
        sorted_result.sort();

        assert_eq!(expected, &sorted_result);
    }

    #[test]
    fn test_dupes_under() {
        let expected =  vec![
        vec![
            fixture("commands/dupes/flac/tracks/fall.free_ranger.flac"),
            fixture("commands/dupes/flac/eps/fall.eds_babe/04.fall.free_ranger.flac"),
            fixture("commands/dupes/flac/eps/various.compilation/11.fall.free_ranger.flac"),
        ],
        vec![
            fixture("commands/dupes/flac/tracks/slint.don_aman.flac"),
            fixture("commands/dupes/flac/albums/pqrs/slint.spiderland_remastered/03.slint.don_aman.flac"),
        ],
    ];

        let mut result = dupes_under(&fixture("commands/dupes/flac")).unwrap();
        result.sort();
        assert_eq!(expected, result);
    }
}
