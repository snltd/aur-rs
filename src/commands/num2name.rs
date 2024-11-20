use crate::utils::metadata::AurMetadata;
use crate::utils::rename;
use std::path::Path;

pub fn run(files: &[String]) -> anyhow::Result<()> {
    rename::rename_files(files, rename_action)
}

pub fn rename_action(file: &Path) -> anyhow::Result<rename::RenameOption> {
    let info = AurMetadata::new(file)?;
    rename::renumber_file(&info)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_run_no_file() {
        if let Err(e) = run(&["/does/not/exist".to_string()]) {
            println!("{}", e);
        }
    }

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
