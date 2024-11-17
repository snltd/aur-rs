use crate::common::metadata::AurMetadata;
use crate::common::rename;
use std::path::Path;

pub fn run(files: &[String]) -> anyhow::Result<()> {
    rename::rename_files(files, rename_action)
}

fn rename_action(file: &Path) -> anyhow::Result<rename::RenameOption> {
    let info = AurMetadata::new(file)?;
    let tags = &info.tags;
    let correct_filename =
        rename::safe_filename(tags.t_num, &tags.artist, &tags.title, &info.filetype);

    if info.filename == correct_filename {
        Ok(None)
    } else {
        let dest = info
            .path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get directory of {:?}", file))?
            .join(correct_filename);

        Ok(Some((file.to_path_buf(), dest.to_path_buf())))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::spec_helper::fixture;

    #[test]
    fn test_run_no_file() {
        if let Err(e) = run(&["/does/not/exist".to_string()]) {
            println!("{}", e);
        }
    }

    #[test]
    fn test_rename_action() {
        let fixture_dir = fixture("info");

        assert_eq!(
            (
                fixture_dir.join("test.flac"),
                fixture_dir.join("06.test_artist.test_title.flac"),
            ),
            rename_action(&fixture("info/test.flac")).unwrap().unwrap()
        );
    }
}
