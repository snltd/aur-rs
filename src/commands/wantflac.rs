use crate::utils::config;
use crate::utils::dir::{expand_dir_list, expand_file_list};
use crate::utils::helpers::check_hierarchy;
use crate::utils::types::{GlobalOpts, WantsList};
use anyhow::ensure;
use camino::{Utf8Path, Utf8PathBuf};
use pathdiff::diff_utf8_paths;
use std::collections::BTreeSet;

pub fn run(root: &Utf8PathBuf, tracks: bool, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let root = root.canonicalize_utf8()?;

    let config = config::load_config(&opts.config)?;

    let wants_list = if tracks {
        filter_by_config(
            find_missing_tracks(&root)?,
            config.get_wantflac_ignore_tracks(),
        )
    } else {
        filter_by_config(
            filter_by_top_level(
                find_missing_albums(&root)?,
                config.get_wantflac_ignore_top_level(),
            ),
            config.get_wantflac_ignore_albums(),
        )
    };

    print_output(wants_list);
    Ok(true)
}

fn filter_by_top_level(list: WantsList, config_list: Option<&WantsList>) -> WantsList {
    match config_list {
        Some(config_list) => list
            .into_iter()
            .filter(|list_element| !config_list.iter().any(|p| list_element.starts_with(p)))
            .collect(),
        None => list,
    }
}

fn filter_by_config(list: WantsList, config_list: Option<&WantsList>) -> WantsList {
    match config_list {
        Some(config_list) => list.difference(config_list).cloned().collect(),
        None => list,
    }
}

fn print_output(wants_list: WantsList) {
    let mut list_vec: Vec<&String> = wants_list.iter().collect();
    list_vec.sort();

    for item in list_vec {
        println!("{}", item);
    }
}

fn find_missing_albums(root: &Utf8Path) -> anyhow::Result<WantsList> {
    check_hierarchy(root)?;

    let mp3_root = root.join("mp3");
    let flac_root = root.join("flac");

    let mp3_names = relative_paths(&expand_dir_list(&[mp3_root.clone()], true), &mp3_root);
    let flac_names = relative_paths(&expand_dir_list(&[flac_root.clone()], true), &flac_root);

    let wanted: BTreeSet<_> = mp3_names
        .difference(&flac_names)
        .map(|s| s.to_owned())
        .collect();
    Ok(wanted)
}

fn relative_paths(dirs: &BTreeSet<Utf8PathBuf>, root: &Utf8PathBuf) -> WantsList {
    dirs.iter()
        .filter_map(|p| diff_utf8_paths(p, root))
        .map(|p| p.to_string())
        .collect()
}

fn simple_filenames(files: &BTreeSet<Utf8PathBuf>) -> WantsList {
    files
        .iter()
        .filter_map(|p| p.file_stem().map(|s| s.to_string()))
        .collect()
}

fn find_missing_tracks(root: &Utf8Path) -> anyhow::Result<WantsList> {
    let mp3_root = root.join("mp3").join("tracks");
    let flac_root = root.join("flac").join("tracks");

    ensure!(mp3_root.exists(), format!("did not find {}", mp3_root));

    ensure!(flac_root.exists(), format!("did not find {}", flac_root));

    let mp3_names = simple_filenames(&expand_file_list(&[mp3_root], true)?);

    let flac_names = simple_filenames(&expand_file_list(&[flac_root], true)?);

    let wanted: BTreeSet<_> = mp3_names
        .difference(&flac_names)
        .map(|s| s.to_owned())
        .collect();
    Ok(wanted)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::fixture;

    #[test]
    fn test_filter_by_config() {
        let config = config::load_config(&fixture("config/test.toml")).unwrap();
        let input: WantsList = BTreeSet::from([
            "albums/abc/artist.album".to_string(),
            "albums/abc/band.record".to_string(),
        ]);

        assert_eq!(
            BTreeSet::from(["albums/abc/band.record".to_string()]),
            filter_by_config(input, config.get_wantflac_ignore_albums())
        );
    }

    #[test]
    fn test_find_missing_albums() {
        let expected = BTreeSet::from([
            "albums/abc/artist.album_1".to_string(),
            "albums/pqrs/singer.second_lp".to_string(),
            "eps/other_band.ep".to_string(),
            "audiobooks".to_string(),
            "audiobooks/writer".to_string(),
            "eps/other_band.ep".to_string(),
            "audiobooks/writer/writer.stories".to_string(),
        ]);

        assert_eq!(
            expected,
            find_missing_albums(&fixture("commands/wantflac")).unwrap()
        );
    }

    #[test]
    fn test_find_missing_albums_with_top_level_filter() {
        let config = config::load_config(&fixture("config/test.toml")).unwrap();

        let expected = BTreeSet::from([
            "albums/abc/artist.album_1".to_string(),
            "albums/pqrs/singer.second_lp".to_string(),
            "eps/other_band.ep".to_string(),
            "eps/other_band.ep".to_string(),
        ]);

        assert_eq!(
            expected,
            filter_by_top_level(
                find_missing_albums(&fixture("commands/wantflac")).unwrap(),
                config.get_wantflac_ignore_top_level()
            )
        );
    }

    #[test]
    fn test_find_missing_tracks() {
        let expected = BTreeSet::from(["artist.tune".to_string(), "band.dirge".to_string()]);

        assert_eq!(
            expected,
            find_missing_tracks(&fixture("commands/wantflac")).unwrap()
        );
    }
}
