use crate::utils::dir::expand_dir_list;
use crate::utils::helpers::check_hierarchy;
use crate::utils::mp3_encoder::{mp3_dir_from, sync_dir, transcode_cmds, TranscodeCmds};
use crate::utils::types::{GlobalOpts, Mp3dirOpts};
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(
    dirlist: &[Utf8PathBuf],
    cmd_opts: &Mp3dirOpts,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let cmds = transcode_cmds()?;
    let root = cmd_opts.root.canonicalize_utf8()?;

    check_hierarchy(&root)?;

    for dir in expand_dir_list(dirlist, cmd_opts.recurse) {
        mp3dir(&dir.canonicalize_utf8()?, &cmds, cmd_opts, opts)?;
    }

    Ok(true)
}

fn mp3dir(
    flac_dir: &Utf8Path,
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
