use crate::utils::config::{self, Config};
use crate::utils::dir;
use crate::utils::helpers;
use crate::utils::mp3_encoder::{self, TranscodeCmds};
use crate::utils::types::{GlobalOpts, Mp3dirOpts};
use camino::Utf8PathBuf;

pub fn run(root_dir: &Utf8PathBuf, bitrate: &str, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let root_dir = root_dir.canonicalize_utf8()?;
    let cmds = mp3_encoder::transcode_cmds()?;
    let conf = config::load_config(&opts.config)?;
    helpers::check_hierarchy(&root_dir)?;
    syncflac(root_dir.join("flac"), bitrate, &conf, &cmds, opts)?;
    Ok(true)
}

fn syncflac(
    flac_root: Utf8PathBuf,
    preset: &str,
    conf: &Config,
    cmds: &TranscodeCmds,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let mut dir_list = dir::expand_dir_list(std::slice::from_ref(&flac_root), true);

    if let Some(ignore_list) = conf.get_syncflac_list() {
        dir_list.retain(|d| !ignore_list.iter().any(|s| d.starts_with(s)));
    }

    let mut synced = 0;

    let cmd_opts = Mp3dirOpts {
        preset: preset.to_owned(),
        force: false,
        recurse: true,
        root: flac_root,
        suffix: false,
    };

    for flac_dir in dir_list.iter() {
        let mp3_dir = mp3_encoder::mp3_dir_from(flac_dir, &cmd_opts);

        if mp3_encoder::sync_dir(flac_dir, &mp3_dir, cmds, &cmd_opts, opts)? {
            synced += 1;
        }
    }

    Ok(synced > 0)
}
