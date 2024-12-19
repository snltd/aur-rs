use crate::utils::dir::pathbuf_set;
use crate::utils::external::find_binary;
use anyhow::anyhow;
use std::path::Path;
use std::process::Command;

pub fn run(files: &[String]) -> anyhow::Result<()> {
    let shnsplit = find_binary("shnsplit")?;

    for f in &pathbuf_set(files) {
        println!("Splitting {}", f.display());
        split_file(f, &shnsplit)?;
    }

    Ok(())
}

fn split_file(file: &Path, shnsplit: &Path) -> anyhow::Result<bool> {
    let cue_file = file.with_extension("cue");

    if !cue_file.exists() {
        return Err(anyhow!("No cue file at '{}'", cue_file.display()));
    }

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
