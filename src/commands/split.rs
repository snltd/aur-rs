use crate::err_if_empty;
use crate::utils::dir;
use crate::utils::external::find_binary;
use anyhow::ensure;
use camino::{Utf8Path, Utf8PathBuf};
use std::process::Command;

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<bool> {
    let shnsplit = find_binary("shnsplit")?;
    let files = dir::pathbuf_set(files);
    err_if_empty!(files);
    let mut ret_code = true;

    for f in files {
        match split_file(&f, &shnsplit) {
            Ok(result) => {
                if !result {
                    eprintln!("Failed to split {f}");
                    ret_code = false;
                }
            }
            Err(e) => {
                eprintln!("Error splitting {f}: {e}");
                ret_code = false;
            }
        }
    }

    Ok(ret_code)
}

fn split_file(file: &Utf8Path, shnsplit: &Utf8Path) -> anyhow::Result<bool> {
    let cue_file = file.with_extension("cue");

    ensure!(cue_file.exists(), "No cue file at '{}'", cue_file);

    let result = Command::new(shnsplit)
        .arg("-f")
        .arg(cue_file)
        .arg("-o")
        .arg("flac")
        .arg("-t")
        .arg("%n.%p.%t")
        .arg(file)
        .status()?;

    Ok(result.success())
}
