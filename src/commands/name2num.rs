use crate::common::metadata::AurMetadata;
use crate::common::tagger::Tagger;
use std::path::{Path, PathBuf};

pub fn run(files: &[String]) -> anyhow::Result<()> {
    for file in files {
        tag_file(&PathBuf::from(file))?;
    }

    Ok(())
}

fn number_from_filename(fname: &str) -> Option<u32> {
    let bits = fname.split('.').collect::<Vec<&str>>();

    match bits.first() {
        Some(bit) => match bit.parse::<u32>() {
            Ok(num) => Some(num),
            Err(_) => None,
        },
        None => None,
    }
}

fn tag_file(file: &Path) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let current_track_number = info.tags.t_num;
    let suggested_track_number = match number_from_filename(info.filename.as_str()) {
        Some(num) => num,
        None => return Ok(false),
    };

    if current_track_number == suggested_track_number {
        return Ok(false);
    }

    let tagger = Tagger::new(info)?;
    tagger.set_t_num(suggested_track_number.to_string().as_str())
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

    #[test]
    fn test_number_from_filename() {
        assert_eq!(3, number_from_filename("03.singer.song.flac").unwrap());
        assert_eq!(99, number_from_filename("99.singer.song.flac").unwrap());
        assert_eq!(None, number_from_filename("singer.song.flac"));
        assert_eq!(None, number_from_filename(".0a.singer.song.flac"));
    }

    #[test]
    fn test_tag_file_flac() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(2, original_info.tags.t_num);
        assert!(tag_file(&file_under_test).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(1, new_info.tags.t_num);

        assert!(!tag_file(&file_under_test).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(1, new_new_info.tags.t_num);
    }

    #[test]
    fn test_tag_file_mp3() {
        let file_name = "03.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(13, original_info.tags.t_num);
        assert!(tag_file(&file_under_test).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(3, new_info.tags.t_num);

        assert!(!tag_file(&file_under_test).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(3, new_new_info.tags.t_num);
    }
}