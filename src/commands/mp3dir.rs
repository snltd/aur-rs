use crate::err_if_empty;
use crate::utils::dir;
use crate::utils::helpers::check_hierarchy;
use crate::utils::mp3_encoder::{TranscodeCmds, mp3_dir_from, sync_dir, transcode_cmds};
use crate::utils::types::{GlobalOpts, Mp3dirOpts};
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(
    dirlist: &[Utf8PathBuf],
    cmd_opts: &Mp3dirOpts,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let cmds = transcode_cmds()?;
    let root = cmd_opts.root.canonicalize_utf8()?;
    let mut ret_code = false;

    check_hierarchy(&root)?;
    let dirs = dir::expand_dir_list(dirlist, cmd_opts.recurse);
    err_if_empty!(dirs);

    for dir in dirs {
        if let Ok(flac_dir) = dir.canonicalize_utf8() {
            if let Err(e) = mp3dir(&flac_dir, &cmds, cmd_opts, opts) {
                eprintln!("Error transcoding in {dir}: {e}");
                ret_code = false;
            }
        } else {
            eprintln!("Error canonicalising {dir}");
            ret_code = false;
        }
    }

    Ok(ret_code)
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
