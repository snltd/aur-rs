use crate::utils::dir;
use crate::utils::types::GlobalOpts;
use anyhow::anyhow;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn run(root: &str, tracks: bool, opts: &GlobalOpts) -> anyhow::Result<()> {
    // println!("looking in {:?}", root);
    // println!("loading config from {:?}", opts.config);
    let root = PathBuf::from(root).canonicalize()?;

    if tracks {
        find_missing_tracks(&root)?;
    } else {
        find_missing_albums(&root)?;
    }

    Ok(())
}

type WantsList = Vec<PathBuf>;

fn find_missing_albums(root: &Path) -> anyhow::Result<WantsList> {
    let mp3_root = root.join("mp3");
    let flac_root = root.join("flac");

    if !mp3_root.exists() {
        return Err(anyhow!(format!("did not find {}", mp3_root.display())));
    }

    if !flac_root.exists() {
        return Err(anyhow!(format!("did not find {}", flac_root.display())));
    }

    let mp3_dirs = dir::expand_dir_list(vec![mp3_root.to_string_lossy().to_string()], true);
    let flac_dirs = dir::expand_dir_list(vec![flac_root.to_string_lossy().to_string()], true);

    let ret = Vec::new();

    Ok(ret)
}

fn find_missing_tracks(root: &Path) -> anyhow::Result<WantsList> {
    let ret = Vec::new();
    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;
}
