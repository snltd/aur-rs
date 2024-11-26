use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::types::GlobalOpts;
use std::path::Path;

pub fn run(files: &[String], _opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(pathbuf_set(files)) {
        retag_file(&f);
    }

    Ok(())
}

fn retag_file(file: &Path) -> anyhow::Result<bool> {
    // let info = AurMetadata::new()?;
    // let tagger = Tagger::new(info)?;

    return Ok(true);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    // cccx[test]
    // fn test_rename_action() {
    //     let fixture_dir = fixture("info");

    //     assert_eq!(
    //         (
    //             fixture_dir.join("test.flac"),
    //             fixture_dir.join("06.test_artist.test_title.flac"),
    //         ),
    //         rename_action(&fixture("info/test.flac")).unwrap().unwrap()
    //     );
    // }
}
