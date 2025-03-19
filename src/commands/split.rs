use crate::utils::dir::pathbuf_set;
use crate::utils::external::find_binary;
use anyhow::ensure;
use camino::{Utf8Path, Utf8PathBuf};
use std::process::Command;

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<()> {
    let shnsplit = find_binary("shnsplit")?;

    for f in &pathbuf_set(files) {
        println!("Splitting {}", f);
        split_file(f, &shnsplit)?;
    }

    Ok(())
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
