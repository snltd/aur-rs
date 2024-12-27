use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use std::path::Path;

pub fn run(files: &[String]) -> anyhow::Result<()> {
    for f in media_files(&pathbuf_set(files)) {
        tag_file(&f)?;
    }

    Ok(())
}

fn tag_file(file: &Path) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let current_artist = &info.tags.artist.clone();

    if current_artist.starts_with("The ") {
        return Ok(false);
    }

    let tagger = Tagger::new(&info)?;
    tagger.set_artist(&format!("The {}", current_artist))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;
    use assert_fs::prelude::*;

    #[test]
    fn test_run_no_file() {
        if let Err(e) = run(&["/does/not/exist".to_string()]) {
            println!("{}", e);
        }
    }

    #[test]
    fn test_tag_file_flac() {
        let file_name = "01.tester.song.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/thes"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Tester ", original_info.tags.artist);
        assert!(tag_file(&file_under_test).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("The Test Artist", new_info.tags.artist);

        assert!(!tag_file(&file_under_test).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("The Tester ", new_new_info.tags.artist);
    }
}
