use crate::common::info::AurMetadata;
use crate::common::tagger::Tagger;
use regex::Regex;
use std::path::{Path, PathBuf};

pub fn run(files: &[String]) -> anyhow::Result<()> {
    let rx = Regex::new(r"^disc_(\d+)$")?;

    for file in files {
        tag_file(&PathBuf::from(file), &rx)?;
    }

    Ok(())
}

fn disc_number(file: &Path, rx: &Regex) -> anyhow::Result<Option<String>> {
    let path = file.canonicalize()?;
    let parent = match path.parent() {
        Some(dir) => dir,
        None => return Ok(None),
    };

    let holding_dir = match parent.file_name() {
        Some(dir) => dir.to_string_lossy().to_string(),
        None => return Ok(None),
    };

    match rx.captures(holding_dir.as_str()) {
        Some(captures) => {
            let disc_num = captures.get(1).map(|m| m.as_str().to_string());
            Ok(disc_num)
        }
        None => Ok(None),
    }
}

fn tag_file(file: &Path, rx: &Regex) -> anyhow::Result<bool> {
    let end_pattern = match disc_number(file, rx)? {
        Some(number) => format!(" (Disc {})", number),
        None => return Ok(false),
    };

    let info = AurMetadata::new(file)?;
    let current_album_name = info.tags.album.clone();

    if current_album_name.ends_with(end_pattern.as_str()) {
        return Ok(false);
    }

    let tagger = Tagger::new(info)?;
    tagger.set_album(format!("{}{}", current_album_name, end_pattern).as_str())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::spec_helper::fixture;
    use assert_fs::prelude::*;

    #[test]
    fn test_run_no_file() {
        if let Err(e) = run(&["/does/not/exist".to_string()]) {
            println!("{}", e);
        }
    }

    fn regex() -> Regex {
        Regex::new(r"^disc_(\d+)$").unwrap()
    }

    #[test]
    fn test_disc_number() {
        assert_eq!(
            Some("3".to_string()),
            disc_number(
                &fixture("commands/albumdisc/disc_3/01.artist.song.mp3"),
                &regex()
            )
            .unwrap()
        );

        assert_eq!(
            None,
            disc_number(&fixture("info/test.flac"), &regex()).unwrap()
        );
    }

    #[test]
    fn test_tag_file() {
        let rx = regex();
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.child("album/disc_3").create_dir_all().unwrap();
        let target = tmp.child("album/disc_3");
        target
            .copy_from(
                fixture("commands/albumdisc/disc_3/"),
                &["01.artist.song.mp3"],
            )
            .unwrap();

        let file_under_test = target.path().join("01.artist.song.mp3");
        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Album".to_string(), original_info.tags.album);
        assert!(tag_file(&file_under_test, &rx).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Album (Disc 3)".to_string(), new_info.tags.album);
        assert!(!tag_file(&file_under_test, &rx).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Album (Disc 3)".to_string(), new_new_info.tags.album);
    }
}
