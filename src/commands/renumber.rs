use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::renumber_file;
use crate::utils::types::{GlobalOpts, RenumberDirection};
use anyhow::ensure;
use camino::Utf8PathBuf;

pub fn run(
    direction: &RenumberDirection,
    delta: u32,
    files: &[Utf8PathBuf],
    opts: &GlobalOpts,
) -> anyhow::Result<()> {
    ensure!(
        (1..=99).contains(&delta),
        "Delta must be from 1 to 99 inclusive"
    );

    let i_delta: i32 = match direction {
        RenumberDirection::Up => delta as i32,
        RenumberDirection::Down => 0 - delta as i32,
    };

    // The casting here is perfectly safe. We can't go outside a very narrow range
    for f in media_files(&pathbuf_set(files)) {
        let info = AurMetadata::new(&f)?;
        let number = info.tags.t_num as i32 + i_delta;

        ensure!(
            (1..=99).contains(&number),
            "Tag number must be from 1 to 99 inclusive"
        );

        renumber_file::update_file(&info, number as u32, opts)?;
    }

    Ok(())
}
