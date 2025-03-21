use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::external::find_binary;
use anyhow::{anyhow, ensure, Context};
use camino::{Utf8Path, Utf8PathBuf};
use rayon::prelude::*;
use std::fs;
use std::process::Command;

struct ReencodeCmds {
    ffmpeg: Utf8PathBuf,
    lame: Utf8PathBuf,
}

pub fn run(files: &[Utf8PathBuf], keep_originals: bool) -> anyhow::Result<bool> {
    let cmds = ReencodeCmds {
        lame: find_binary("lame")?,
        ffmpeg: find_binary("ffmpeg")?,
    };

    media_files(&pathbuf_set(files))
        .par_iter()
        .try_for_each(|f| reencode_file(f, keep_originals, &cmds))?;

    Ok(true)
}

fn reencode_file(file: &Utf8Path, keep_originals: bool, cmds: &ReencodeCmds) -> anyhow::Result<()> {
    println!("{}", file);

    let ext = file
        .extension()
        .context(format!("cannot extract extension for {}", file))?;

    let stem = file
        .file_stem()
        .context(format!("cannot extract file stem for {}", file))?;

    let dir = file
        .parent()
        .context(format!("cannot get directory for {}", file))?;

    let target_file = dir.join(format!("{}.reencoded.{}", stem, ext));

    let success = match ext {
        "flac" => reencode_flac(file, &target_file, &cmds.ffmpeg)?,
        "mp3" => reencode_mp3(file, &target_file, &cmds.lame)?,
        _ => return Err(anyhow!("unexpected filetype: {}", ext)),
    };

    ensure!(success, "REENCODE FAILED");

    if keep_originals {
        Ok(())
    } else {
        fs::rename(target_file, file).map_err(|e| anyhow::anyhow!(e))
    }
}

fn reencode_flac(
    src_file: &Utf8Path,
    target_file: &Utf8Path,
    ffmpeg: &Utf8Path,
) -> anyhow::Result<bool> {
    let result = Command::new(ffmpeg)
        .arg("-hide_banner")
        .arg("-loglevel")
        .arg("error")
        .arg("-y")
        .arg("-i")
        .arg(src_file)
        .arg("-compression_level")
        .arg("9")
        .arg(target_file)
        .status()?;

    Ok(result.success())
}

fn reencode_mp3(
    src_file: &Utf8Path,
    target_file: &Utf8Path,
    lame: &Utf8Path,
) -> anyhow::Result<bool> {
    let result = Command::new(lame)
        .arg("-q2")
        .arg("--vbr-new")
        .arg("--preset")
        .arg("128")
        .arg("--silent")
        .arg(src_file)
        .arg(target_file)
        .status()?;

    Ok(result.success())
}
