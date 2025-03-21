use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::expected_tags;
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashSet;

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<bool> {
    for f in media_files(&pathbuf_set(files)) {
        strip_file(&f)?;
    }

    Ok(true)
}

fn strip_file(file: &Utf8Path) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let tagger = Tagger::new(&info)?;
    remove_artwork(&info, &tagger)?;
    remove_tags(&info, &tagger)
}

fn remove_tags(info: &AurMetadata, tagger: &Tagger) -> anyhow::Result<bool> {
    let expected_tags = expected_tags(&info.filetype)?;
    let rawtag_keys: HashSet<String> = info.rawtags.iter().map(|(k, _v)| k).cloned().collect();
    let mut to_remove: Vec<String> = rawtag_keys.difference(&expected_tags).cloned().collect();
    to_remove.sort();

    println!(
        "Strip: {} :: {}",
        info.path,
        to_remove
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(", ")
    );

    tagger.remove_tags(&to_remove)
}

fn remove_artwork(info: &AurMetadata, tagger: &Tagger) -> anyhow::Result<bool> {
    if info.has_picture {
        println!("Strip: {} :: embedded artwork", info.path);
        tagger.remove_artwork()
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::{fixture, TempDirExt};
    use assert_fs::prelude::*;

    #[test]
    fn test_strip_flac() {
        let file_name = "01.tester.not_stripped.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/strip"), &[file_name])
            .unwrap();
        let file_under_test = tmp.utf8_path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(9, original_info.rawtags.len());
        assert!(strip_file(&file_under_test).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(6, new_info.rawtags.len());

        assert!(!strip_file(&file_under_test).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(6, new_new_info.rawtags.len());
    }

    #[test]
    fn test_strip_mp3() {
        let file_name = "02.tester.not_stripped.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/strip"), &[file_name])
            .unwrap();
        let file_under_test = tmp.utf8_path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(12, original_info.rawtags.len());
        assert!(strip_file(&file_under_test).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(6, new_info.rawtags.len());

        assert!(!strip_file(&file_under_test).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(6, new_new_info.rawtags.len());
    }
}
