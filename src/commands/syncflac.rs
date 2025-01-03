use crate::utils::config::{load_config, Config};
use crate::utils::dir::expand_dir_list;
use crate::utils::helpers::check_hierarchy;
use crate::utils::lame_wrapper::{
    clean_up_file, make_clean_up_list, make_transcode_list, transcode_cmds, transcode_file,
    TranscodeCmds,
};
use crate::utils::types::GlobalOpts;
use crate::verbose;
use anyhow::anyhow;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

pub fn run(root_dir: &str, opts: &GlobalOpts) -> anyhow::Result<()> {
    let cmds = transcode_cmds()?;
    let root = PathBuf::from(root_dir).canonicalize()?;
    let conf = load_config(&opts.config)?;

    check_hierarchy(&root)?;
    syncflac(root.join("flac"), &conf, &cmds, opts)?;
    Ok(())
}

fn syncflac(
    flac_root: PathBuf,
    conf: &Config,
    cmds: &TranscodeCmds,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let mut dir_list = expand_dir_list(&[flac_root], true);

    if let Some(ignore_list) = conf.get_syncflac_list() {
        dir_list.retain(|d| {
            !ignore_list
                .iter()
                .any(|s| d.to_string_lossy().to_string().starts_with(s))
        });
    }

    let mut synced = 0;

    for flac_dir in dir_list.iter() {
        let mp3_dir = mp3_dir_from(flac_dir);
        if sync_dir(flac_dir, &mp3_dir, cmds, opts)? {
            synced += 1;
        }
    }

    Ok(synced > 0)
}

fn mp3_dir_from(flac_dir: &Path) -> PathBuf {
    PathBuf::from(
        flac_dir
            .to_string_lossy()
            .to_string()
            .replace("/flac", "/mp3"),
    )
}

fn sync_dir(
    flac_dir: &Path,
    mp3_dir: &Path,
    cmds: &TranscodeCmds,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    if flac_dir == mp3_dir {
        return Err(anyhow!(
            "FLAC and MP3 directories have the same path: {}",
            flac_dir.display()
        ));
    }

    if !mp3_dir.exists() && !opts.noop {
        verbose!(opts, "Creating directory {}", mp3_dir.display());
        create_dir_all(mp3_dir)?;
    }

    for t in make_transcode_list(flac_dir, mp3_dir)? {
        transcode_file(&t, cmds, opts)?;
    }

    if mp3_dir.exists() && mp3_dir.file_name().unwrap() != "tracks" {
        // it might not be there if we just no-oped, and we allow tracks/ to be different
        for f in make_clean_up_list(flac_dir, mp3_dir)? {
            clean_up_file(&f, opts)?;
        }
    }

    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mp3_dir_from() {
        assert_eq!(
            PathBuf::from("/storage/mp3/tracks"),
            mp3_dir_from(&PathBuf::from("/storage/flac/tracks")),
        );
    }
}
