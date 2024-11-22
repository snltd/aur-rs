use crate::utils::metadata::AurMetadata;
use crate::utils::rename::number_from_filename;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use std::path::{Path, PathBuf};

pub fn run(files: &[String], opts: &GlobalOpts) -> anyhow::Result<()> {
    for file in files {
        tag_file(&PathBuf::from(file), opts)?;
    }

    Ok(())
}

fn tag_file(file: &Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let current_track_number = info.tags.t_num;
    let suggested_track_number = match number_from_filename(info.filename.as_str()) {
        Some((_path, num)) => num,
        None => {
            verbose!(opts, "Filename has correct track number");
            return Ok(false);
        }
    };

    if current_track_number == suggested_track_number {
        verbose!(opts, "Track number tag is correct");
        return Ok(false);
    }

    let tagger = Tagger::new(&info)?;
    tagger.set_t_num(suggested_track_number.to_string().as_str())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::{defopts, fixture};
    use assert_fs::prelude::*;

    #[test]
    fn test_tag_file_flac() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(2, original_info.tags.t_num);
        assert!(tag_file(&file_under_test, &defopts()).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(1, new_info.tags.t_num);

        assert!(!tag_file(&file_under_test, &defopts()).unwrap());
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
        assert!(tag_file(&file_under_test, &defopts()).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(3, new_info.tags.t_num);

        assert!(!tag_file(&file_under_test, &defopts()).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(3, new_new_info.tags.t_num);
    }
}
