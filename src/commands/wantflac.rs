use crate::utils::dir::{expand_dir_list, expand_file_list};
use crate::utils::types::GlobalOpts;
use anyhow::anyhow;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

type WantsList = HashSet<String>;

pub fn run(root: &str, tracks: bool, opts: &GlobalOpts) -> anyhow::Result<()> {
    let root = PathBuf::from(root).canonicalize()?;

    let wants_list = if tracks {
        find_missing_tracks(&root)?
    } else {
        find_missing_albums(&root)?
    };

    print_output(wants_list);

    Ok(())
}

fn print_output(wants_list: WantsList) {
    let mut list_vec: Vec<&String> = wants_list.iter().collect();
    list_vec.sort();

    for item in list_vec {
        println!("{}", item);
    }
}

fn find_missing_albums(root: &Path) -> anyhow::Result<WantsList> {
    let mp3_root = root.join("mp3");
    let flac_root = root.join("flac");

    if !mp3_root.exists() {
        return Err(anyhow!(format!("did not find {}", mp3_root.display())));
    }

    if !flac_root.exists() {
        return Err(anyhow!(format!("did not find {}", flac_root.display())));
    }

    let mp3_names = relative_paths(&expand_dir_list(&[mp3_root.clone()], true), &mp3_root);
    let flac_names = relative_paths(&expand_dir_list(&[flac_root.clone()], true), &flac_root);

    let wanted: HashSet<_> = mp3_names
        .difference(&flac_names)
        .map(|s| s.to_owned())
        .collect();
    Ok(wanted)
}

fn relative_paths(dirs: &HashSet<PathBuf>, root: &PathBuf) -> WantsList {
    dirs.iter()
        .filter_map(|p| {
            p.strip_prefix(root)
                .ok()
                .and_then(|s| s.to_str().map(String::from))
        })
        .collect()
}

fn simple_filenames(files: &HashSet<PathBuf>) -> WantsList {
    files
        .iter()
        .filter_map(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()))
        .collect()
}

fn find_missing_tracks(root: &Path) -> anyhow::Result<WantsList> {
    let mp3_root = root.join("mp3").join("tracks");
    let flac_root = root.join("flac").join("tracks");

    if !mp3_root.exists() {
        return Err(anyhow!(format!("did not find {}", mp3_root.display())));
    }

    if !flac_root.exists() {
        return Err(anyhow!(format!("did not find {}", flac_root.display())));
    }

    let mp3_names = simple_filenames(&expand_file_list(
        &[mp3_root.to_string_lossy().to_string()],
        true,
    )?);

    let flac_names = simple_filenames(&expand_file_list(
        &[flac_root.to_string_lossy().to_string()],
        true,
    )?);

    let wanted: HashSet<_> = mp3_names
        .difference(&flac_names)
        .map(|s| s.to_owned())
        .collect();
    Ok(wanted)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;
    // use assert_unordered::assert_eq_unordered;

    #[test]
    fn test_find_missing_albums() {
        let expected = HashSet::from([
            "albums/abc/artist.album_1".to_string(),
            "albums/pqrs/singer.second_lp".to_string(),
            "eps/other_band.ep".to_string(),
        ]);

        assert_eq!(
            expected,
            find_missing_albums(&fixture("commands/wantflac")).unwrap()
        );
    }

    #[test]
    fn test_find_missing_tracks() {
        let expected = HashSet::from(["artist.tune".to_string(), "band.dirge".to_string()]);

        assert_eq!(
            expected,
            find_missing_tracks(&fixture("commands/wantflac")).unwrap()
        );
    }
}
