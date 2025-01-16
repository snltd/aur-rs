use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use anyhow::anyhow;
use std::io::{self, Write};
use std::path::Path;

pub fn run(files: &[String], tag: &str, opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(&pathbuf_set(files)) {
        let value = read_value(&f)?;
        tag_file(&f, tag, &value, opts)?;
    }

    Ok(())
}

fn read_value(file: &Path) -> anyhow::Result<String> {
    let basename = match file.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(anyhow!("could not get file name")),
    };

    print!("{} > ", basename);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let input = buffer.to_owned().trim().to_string();
    Ok(input)
}

fn tag_file(file: &Path, tag: &str, value: &str, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let tagger = Tagger::new(&info)?;

    tagger.set_tag(tag, value, opts.quiet)
}
