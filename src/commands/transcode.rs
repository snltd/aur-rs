use crate::utils::dir::pathbuf_set;
use crate::utils::external::find_binary;
use crate::utils::types::{GlobalOpts, TranscodeOptions};
use crate::verbose;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
use std::process::Command;

pub fn run(
    files: &[Utf8PathBuf],
    format: &str,
    cmd_opts: &TranscodeOptions,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let ffmpeg = find_binary("ffmpeg")?;
    let mut ret = true;

    for f in &pathbuf_set(files) {
        if !transcode_file(f, format, cmd_opts, opts, &ffmpeg)? {
            ret = false
        }
    }

    Ok(ret)
}

fn transcode_file(
    file: &Utf8Path,
    format: &str,
    cmd_opts: &TranscodeOptions,
    opts: &GlobalOpts,
    ffmpeg: &Utf8Path,
) -> anyhow::Result<bool> {
    let target_file = file.with_extension(format);

    if target_file.exists() && !cmd_opts.force {
        verbose!(opts, "target '{}' exists. Use -f to overwrite", target_file);
        return Ok(false);
    }

    println!("{} -> {}", file, target_file);

    let result = Command::new(ffmpeg)
        .arg("-hide_banner")
        .arg("-loglevel")
        .arg("error")
        .arg("-y")
        .arg("-i")
        .arg(file)
        .arg(target_file)
        .status()?;

    if cmd_opts.remove_originals {
        verbose!(opts, "Removing {}", file);
        fs::remove_file(file)?;
    }

    Ok(result.success())
}
