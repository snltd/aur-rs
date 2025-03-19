use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};
use regex::Regex;

pub fn run(files: &[Utf8PathBuf], global_opts: &GlobalOpts) -> anyhow::Result<()> {
    let rx = Regex::new(r"^disc_(\d+)$")?;

    for file in media_files(&pathbuf_set(files)) {
        tag_file(&file, &rx, global_opts)?;
    }

    Ok(())
}

fn tag_file(file: &Utf8Path, rx: &Regex, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let end_pattern = match disc_number(file, rx)? {
        Some(number) => format!(" (Disc {})", number),
        None => return Err(anyhow!("{} is not in a disc_n directory", file)),
    };

    let info = AurMetadata::new(file)?;
    let current_album_name = &info.tags.album;

    if current_album_name.ends_with(end_pattern.as_str()) {
        verbose!(opts, "album tag already ends with {}", end_pattern);
        return Ok(false);
    }

    let tagger = Tagger::new(&info)?;
    tagger.set_album(
        format!("{}{}", current_album_name, end_pattern).as_str(),
        opts.quiet,
    )
}

fn disc_number(file: &Utf8Path, rx: &Regex) -> anyhow::Result<Option<String>> {
    let path = file.canonicalize_utf8()?;
    let parent = match path.parent() {
        Some(dir) => dir,
        None => return Ok(None),
    };

    let holding_dir = match parent.file_name() {
        Some(dir) => dir.to_string(),
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::{defopts, fixture};
    use assert_fs::prelude::*;

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

        let file_under_test = Utf8Path::from_path(target.path())
            .unwrap()
            .join("01.artist.song.mp3");
        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Album", original_info.tags.album);
        assert!(tag_file(&file_under_test, &rx, &defopts()).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Album (Disc 3)", new_info.tags.album);
        assert!(!tag_file(&file_under_test, &rx, &defopts()).unwrap());
        let new_new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Album (Disc 3)", new_new_info.tags.album);
    }
}
