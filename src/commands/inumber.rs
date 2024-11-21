use crate::utils::metadata::AurMetadata;
use crate::utils::renumber_file;
use anyhow::anyhow;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn run(files: &[String]) -> anyhow::Result<()> {
    for file in files {
        let path = PathBuf::from(file);
        let number = read_number(&path)?;
        let info = AurMetadata::new(&path)?;

        if !(1..=99).contains(&number) {
            return Err(anyhow!("Tag number must be from 1 to 99 inclusive"));
        }

        renumber_file::update_file(&info, number)?;
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
