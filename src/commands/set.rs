use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(
    tag: &str,
    value: &str,
    files: &[Utf8PathBuf],
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    for f in media_files(&pathbuf_set(files)) {
        tag_file(tag, value, &f, opts)?;
    }

    Ok(true)
}

fn tag_file(key: &str, value: &str, file: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    Tagger::new(&info)?.set_tag(key, value, opts.quiet)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::{defopts, fixture};
    use camino_tempfile_ext::prelude::*;

    #[test]
    fn test_tag_file_flac() {
        let file_name = "01.tester.song.flac";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/set"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("Tester", original_info.tags.artist);
        assert!(!tag_file("artist", "Tester", &file_under_test, &defopts()).unwrap());

        let new_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("Tester", new_info.tags.artist);
        assert!(tag_file("artist", "New Artist", &file_under_test, &defopts()).unwrap());

        let new_new_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("New Artist", new_new_info.tags.artist);
    }

    #[test]
    fn test_tag_file_mp3() {
        let file_name = "02.tester.song.mp3";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/set"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("Test Set", original_info.tags.album);
        assert!(!tag_file("album", "Test Set", &file_under_test, &defopts()).unwrap());

        let new_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("Test Set", new_info.tags.album);
        assert!(tag_file("album", "New Album", &file_under_test, &defopts()).unwrap());

        let new_new_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("New Album", new_new_info.tags.album);
    }
}
