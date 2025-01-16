use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::renumber_file;
use crate::utils::types::GlobalOpts;
use anyhow::anyhow;
use std::io::{self, Write};
use std::path::Path;

pub fn run(files: &[String], opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(&pathbuf_set(files)) {
        let number = read_number(&f)?;
        let info = AurMetadata::new(&f)?;

        if !(1..=99).contains(&number) {
            return Err(anyhow!("Tag number must be from 1 to 99 inclusive"));
        }

        renumber_file::update_file(&info, number, opts)?;
    }

    Ok(())
}

fn read_number(file: &Path) -> anyhow::Result<u32> {
    let basename = match file.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => return Err(anyhow!("could not get file name")),
    };

    print!("{} > ", basename);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let num = buffer.to_owned().trim().to_string().parse::<u32>()?;
    Ok(num)
}
