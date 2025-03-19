use crate::utils::config::{load_config, Config};
use crate::utils::dir::expand_dir_list;
use crate::utils::helpers::check_hierarchy;
use crate::utils::mp3_encoder::{mp3_dir_from, sync_dir, transcode_cmds, TranscodeCmds};
use crate::utils::types::{GlobalOpts, Mp3dirOpts};
use camino::Utf8PathBuf;

pub fn run(root_dir: &Utf8PathBuf, bitrate: &str, opts: &GlobalOpts) -> anyhow::Result<()> {
    let root_dir = root_dir.canonicalize_utf8()?;
    let cmds = transcode_cmds()?;
    let conf = load_config(&opts.config)?;

    check_hierarchy(&root_dir)?;
    syncflac(root_dir.join("flac"), bitrate, &conf, &cmds, opts)?;
    Ok(())
}

fn syncflac(
    flac_root: Utf8PathBuf,
    bitrate: &str,
    conf: &Config,
    cmds: &TranscodeCmds,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let mut dir_list = expand_dir_list(&[flac_root.clone()], true);

    if let Some(ignore_list) = conf.get_syncflac_list() {
        dir_list.retain(|d| !ignore_list.iter().any(|s| d.starts_with(s)));
    }

    let mut synced = 0;

    let cmd_opts = Mp3dirOpts {
        bitrate: bitrate.into(),
        force: false,
        recurse: true,
        root: flac_root,
        suffix: false,
    };

    for flac_dir in dir_list.iter() {
        let mp3_dir = mp3_dir_from(flac_dir, &cmd_opts);

        if sync_dir(flac_dir, &mp3_dir, cmds, &cmd_opts, opts)? {
            synced += 1;
        }
    }

    Ok(synced > 0)
}
