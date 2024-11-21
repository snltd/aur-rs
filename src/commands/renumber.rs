use crate::utils::metadata::AurMetadata;
use crate::utils::renumber_file;
use crate::utils::types::RenumberDirection;
use anyhow::anyhow;
use std::path::PathBuf;

pub fn run(direction: &RenumberDirection, delta: u32, files: &[String]) -> anyhow::Result<()> {
    if !(1..=99).contains(&delta) {
        return Err(anyhow!("Delta must be from 1 to 99 inclusive"));
    }

    let i_delta: i32 = match direction {
        RenumberDirection::Up => delta as i32,
        RenumberDirection::Down => 0 - delta as i32,
    };

    // The number stuff here is perfectly safe. We can't go outside a very narrow range
    for file in files {
        let path = PathBuf::from(file);
        let info = AurMetadata::new(&path)?;
        let number = info.tags.t_num as i32 + i_delta;

        if !(1..=99).contains(&number) {
            return Err(anyhow!("Tag number must be from 1 to 99 inclusive"));
        }

        renumber_file::update_file(&info, number as u32)?;
    }

    Ok(())
}
