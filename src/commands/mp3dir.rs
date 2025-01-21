use crate::utils::dir::expand_dir_list;
use crate::utils::helpers::check_hierarchy;
use crate::utils::mp3_encoder::{mp3_dir_from, sync_dir, transcode_cmds, TranscodeCmds};
use crate::utils::types::{GlobalOpts, Mp3dirOpts};
use std::path::{Path, PathBuf};

pub fn run(dirlist: &[String], cmd_opts: &Mp3dirOpts, opts: &GlobalOpts) -> anyhow::Result<()> {
    let cmds = transcode_cmds()?;
    let root = PathBuf::from(&cmd_opts.root).canonicalize()?;

    check_hierarchy(&root)?;

    let dirs_to_list: Vec<PathBuf> = dirlist.iter().map(PathBuf::from).collect();

    for dir in expand_dir_list(&dirs_to_list, cmd_opts.recurse) {
        let dir = dir.canonicalize()?;
        mp3dir(&dir, &cmds, cmd_opts, opts)?;
    }

    Ok(())
}

fn mp3dir(
    flac_dir: &Path,
    cmds: &TranscodeCmds,
    cmd_opts: &Mp3dirOpts,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let mut synced = 0;
    let mp3_dir = mp3_dir_from(flac_dir, cmd_opts);

    if sync_dir(flac_dir, &mp3_dir, cmds, cmd_opts, opts)? {
        synced += 1;
    }

    Ok(synced > 0)
}
