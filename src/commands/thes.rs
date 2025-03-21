use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(files: &[Utf8PathBuf], opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(&pathbuf_set(files)) {
        tag_file(&f, opts)?;
    }

    Ok(())
}

fn tag_file(file: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let current_artist = &info.tags.artist.clone();

    if current_artist.starts_with("The ") {
        return Ok(false);
    }

    let tagger = Tagger::new(&info)?;
    tagger.set_artist(&format!("The {}", current_artist), opts.quiet)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::{defopts, fixture, TempDirExt};
    use assert_fs::prelude::*;

    #[test]
    fn test_tag_file_flac() {
        let file_name = "01.tester.song.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/thes"), &[file_name])
            .unwrap();
        let file_under_test = tmp.utf8_path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Tester", original_info.tags.artist);
        assert!(tag_file(&file_under_test, &defopts()).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("The Tester", new_info.tags.artist);

        assert!(!tag_file(&file_under_test, &defopts()).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("The Tester", new_new_info.tags.artist);
    }
}
