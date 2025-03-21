use crate::utils::dir::{expand_file_list, media_files};
use anyhow::ensure;
use camino::{Utf8Path, Utf8PathBuf};
use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::sync::LazyLock;

type Dupes = Vec<Vec<Utf8PathBuf>>;

static NO_LEADING_NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d\d\.(.*)$").unwrap());

pub fn run(root_dir: &Utf8PathBuf) -> anyhow::Result<bool> {
    let dupes = dupes_under(root_dir)?;
    dupes.iter().for_each(|d| println!("{}", format_dupes(d)));
    Ok(dupes.is_empty())
}

fn format_dupes(dupe_cluster: &[Utf8PathBuf]) -> String {
    let mut ret: String = format!("{}", dupe_cluster.first().unwrap());
    dupe_cluster[1..]
        .iter()
        .for_each(|d| ret.push_str(&format!("\n  {}", d)));
    ret.push('\n');

    ret
}

fn file_hash(paths: &BTreeSet<Utf8PathBuf>) -> HashMap<String, Vec<Utf8PathBuf>> {
    let mut ret: HashMap<String, Vec<Utf8PathBuf>> = HashMap::new();

    for p in paths {
        if let Some(fname) = filename_from_file(p) {
            ret.entry(fname).or_default().push(p.clone());
        }
    }

    ret
}

fn dupes_under(dir: &Utf8Path) -> anyhow::Result<Dupes> {
    for d in ["tracks", "eps", "albums"] {
        let required_dir = dir.join(d);
        ensure!(required_dir.exists(), format!("{} not found", required_dir));
    }

    let needle_files = expand_file_list(&[dir.join("tracks")], true)?;
    let haystack_files = expand_file_list(&[dir.join("albums"), dir.join("eps")], true)?;
    let needle_hash = file_hash(&media_files(&needle_files));
    let haystack_hash = file_hash(&media_files(&haystack_files));

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

fn filename_from_file(path: &Utf8Path) -> Option<String> {
    if let Some(name) = path.file_name() {
        if let Some(c) = NO_LEADING_NUMBER.captures(name) {
            return Some(c[1].to_string());
        }
        return Some(name.to_string());
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    fn test_missing_arg() {
        assert!(dupes_under(&Utf8PathBuf::from("/does/not/exist")).is_err());
    }

    #[test]
    fn test_filename_from_file() {
        assert_eq!(
            "singer.song.flac",
            filename_from_file(&Utf8PathBuf::from(
                "/path/to/singer.album/02.singer.song.flac"
            ))
            .unwrap()
        );

        assert_eq!(
            "singer.song.flac",
            filename_from_file(&Utf8PathBuf::from("/path/to/singer.album/singer.song.flac"))
                .unwrap()
        );
    }

    #[test]
    fn test_file_hash() {
        let mut input = BTreeSet::new();
        input.insert(Utf8PathBuf::from(
            "/flac/eps/singer.ep/01.singer.song_1.flac",
        ));
        input.insert(Utf8PathBuf::from(
            "/flac/eps/singer.ep/02.singer.song_2.flac",
        ));
        input.insert(Utf8PathBuf::from(
            "/flac/albums/pqrs/singer.lp/06.singer.song_1.flac",
        ));
        input.insert(Utf8PathBuf::from("/flac/eps/singer.ep/front.jpg"));

        let result = file_hash(&input);

        assert_eq!(3, result.len());
        assert_eq!(
            &vec![Utf8PathBuf::from(
                "/flac/eps/singer.ep/02.singer.song_2.flac"
            )],
            result.get("singer.song_2.flac").unwrap()
        );

        let expected = &vec![
            Utf8PathBuf::from("/flac/albums/pqrs/singer.lp/06.singer.song_1.flac"),
            Utf8PathBuf::from("/flac/eps/singer.ep/01.singer.song_1.flac"),
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
