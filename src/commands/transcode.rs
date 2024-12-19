use crate::utils::dir::pathbuf_set;
use crate::utils::external::find_binary;
use crate::utils::types::{GlobalOpts, TranscodeOptions};
use crate::verbose;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run(
    files: &[String],
    format: &str,
    cmd_opts: &TranscodeOptions,
    opts: &GlobalOpts,
) -> anyhow::Result<()> {
    let ffmpeg = find_binary("ffmpeg")?;

    for f in &pathbuf_set(files) {
        transcode_file(f, format, cmd_opts, opts, &ffmpeg)?;
    }

    Ok(())
}

fn transcode_file(
    file: &Path,
    format: &str,
    cmd_opts: &TranscodeOptions,
    opts: &GlobalOpts,
    ffmpeg: &Path,
) -> anyhow::Result<bool> {
    let target_file = file.with_extension(format);

    if target_file.exists() && !cmd_opts.force {
        verbose!(
            opts,
            "target '{}' exists. Use -f to overwrite",
            target_file.display()
        );
        return Ok(false);
    }

    println!("{} -> {}", file.display(), target_file.display());

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
        verbose!(opts, "Removing {}", file.display());
        fs::remove_file(file)?;
    }

    Ok(result.success())
}
