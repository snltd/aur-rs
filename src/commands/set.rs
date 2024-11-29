use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use std::path::Path;

pub fn run(tag: &str, value: &str, files: &[String]) -> anyhow::Result<()> {
    for f in media_files(pathbuf_set(files)) {
        tag_file(tag, value, &f)?;
    }

    Ok(())
}

fn tag_file(key: &str, value: &str, file: &Path) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let tagger = Tagger::new(&info)?;
    tagger.set_tag(key, value)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;
    use assert_fs::prelude::*;

    #[test]
    fn test_run_no_file() {
        if let Err(e) = run("title", "Test Title", &["/does/not/exist".to_string()]) {
            println!("{}", e);
        }
    }

    #[test]
    fn test_tag_file_flac() {
        let file_name = "test.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file_name]).unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Artist", original_info.tags.artist);
        assert!(!tag_file("artist", "Test Artist", &file_under_test).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Artist", new_info.tags.artist);

        assert!(tag_file("artist", "New Artist", &file_under_test).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("New Artist", new_new_info.tags.artist);
    }

    #[test]
    fn test_tag_file_mp3() {
        let file_name = "test.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file_name]).unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Album", original_info.tags.album);
        assert!(!tag_file("album", "Test Album", &file_under_test).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Album", new_info.tags.album);

        assert!(tag_file("album", "New Album", &file_under_test).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("New Album", new_new_info.tags.album);
    }
}
