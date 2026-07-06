use crate::utils::dir;
use crate::utils::metadata::AurMetadata;
use crate::utils::rename::number_from_filename;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use crate::{err_if_empty, verbose};
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(files: &[Utf8PathBuf], opts: &GlobalOpts) -> anyhow::Result<bool> {
    let files = dir::media_files(&dir::pathbuf_set(files));
    let mut ret_code = true;
    err_if_empty!(files);

    for file in files {
        if let Err(e) = tag_file(&file, opts) {
            eprintln!("Error tagging {file}: {e}");
            ret_code = false;
        }
    }

    Ok(ret_code)
}

fn tag_file(file: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let current_track_number = info.tags.t_num;
    let suggested_track_number = match number_from_filename(&info.filename) {
        Some((_path, num)) => num,
        None => {
            println!("Could not get number from {}", &info.filename);
            return Ok(false);
        }
    };

    if current_track_number == suggested_track_number {
        verbose!(opts, "Track number tag is correct");
        return Ok(false);
    }

    let tagger = Tagger::new(&info)?;
    tagger.set_t_num(&suggested_track_number.to_string(), opts.quiet)
}

#[cfg(test)]
mod test {
    use super::*;
    use camino_tempfile_ext::prelude::*;
    use snltest::fixture;

    #[test]
    fn test_tag_file_flac() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(2, original_info.tags.t_num);
        assert!(tag_file(&file_under_test, &GlobalOpts::default()).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(1, new_info.tags.t_num);

        assert!(!tag_file(&file_under_test, &GlobalOpts::default()).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!(1, new_new_info.tags.t_num);
    }

    #[test]
    fn test_tag_file_mp3() {
        let file_name = "03.test_artist.test_title.mp3";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let original_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!(13, original_info.tags.t_num);
        assert!(tag_file(&file_under_test, &GlobalOpts::default()).unwrap());

        let new_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!(3, new_info.tags.t_num);
        assert!(!tag_file(&file_under_test, &GlobalOpts::default()).unwrap());

        let new_new_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!(3, new_new_info.tags.t_num);
    }
}
