use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::external::find_binary;
use anyhow::anyhow;
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

struct ReencodeCmds {
    ffmpeg: PathBuf,
    lame: PathBuf,
}

pub fn run(files: &[String], keep_originals: bool) -> anyhow::Result<()> {
    let cmds = ReencodeCmds {
        lame: find_binary("lame")?,
        ffmpeg: find_binary("ffmpeg")?,
    };

    media_files(&pathbuf_set(files))
        .par_iter()
        .try_for_each(|f| reencode_file(f, keep_originals, &cmds))?;

    Ok(())
}

fn reencode_file(file: &Path, keep_originals: bool, cmds: &ReencodeCmds) -> anyhow::Result<()> {
    println!("{}", file.display());

    let ext = match file.extension() {
        Some(osstr) => osstr.to_string_lossy().to_string(),
        None => return Err(anyhow!("cannot extract extension from {}", file.display())),
    };

    let stem = match file.file_stem() {
        Some(osstr) => osstr.to_string_lossy(),
        None => return Err(anyhow!("cannot extract stem from {}", file.display())),
    };

    let dir = match file.parent() {
        Some(dir) => dir,
        None => return Err(anyhow!("cannot get directory of {}", file.display())),
    };

    let target_file = dir.join(format!("{}.reencoded.{}", stem, ext));

    let success = match ext.as_str() {
        "flac" => reencode_flac(file, &target_file, &cmds.ffmpeg)?,
        "mp3" => reencode_mp3(file, &target_file, &cmds.lame)?,
        _ => return Err(anyhow!("unexpected filetype: {}", ext)),
    };

    if !success {
        return Err(anyhow!("reencode failed"));
    }

    if keep_originals {
        Ok(())
    } else {
        fs::rename(target_file, file).map_err(|e| anyhow::anyhow!(e))
    }
}

fn reencode_flac(src_file: &Path, target_file: &Path, ffmpeg: &Path) -> anyhow::Result<bool> {
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

fn reencode_mp3(src_file: &Path, target_file: &Path, lame: &Path) -> anyhow::Result<bool> {
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
