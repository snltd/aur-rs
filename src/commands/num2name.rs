use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::rename;
use crate::utils::types::{GlobalOpts, RenameOption};
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(files: &[Utf8PathBuf], opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(&pathbuf_set(files)) {
        if let Some(action) = rename_action(&f)? {
            rename::rename(action, opts.noop)?;
        }
    }

    Ok(())
}

pub fn rename_action(file: &Utf8Path) -> anyhow::Result<RenameOption> {
    let info = AurMetadata::new(file)?;
    rename::renumber_file(&info)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::fixture;

    #[test]
    fn test_rename_action() {
        let fixture_dir = fixture("commands/name2num");

        assert_eq!(
            (
                fixture_dir.join("01.test_artist.test_title.flac"),
                fixture_dir.join("02.test_artist.test_title.flac"),
            ),
            rename_action(&fixture("commands/name2num/01.test_artist.test_title.flac"))
                .unwrap()
                .unwrap()
        );
    }
}
