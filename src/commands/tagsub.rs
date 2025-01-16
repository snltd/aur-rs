use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use regex::Regex;
use std::path::Path;

pub fn run(
    files: &[String],
    tag: &str,
    from: &str,
    to: &str,
    opts: &GlobalOpts,
) -> anyhow::Result<()> {
    let rx = Regex::new(from)?;

    for file in media_files(&pathbuf_set(files)) {
        process_file(&file, tag, &rx, to, opts)?;
    }

    Ok(())
}

fn process_file(
    file: &Path,
    tag: &str,
    rx: &Regex,
    to: &str,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let old_value = info.get_tag(tag)?;
    match new_tag(&old_value, rx, to)? {
        Some(new_value) => {
            verbose!(opts, "{}: {} -> {}", file.display(), old_value, new_value);
            if opts.noop {
                println!("{}: {} -> {}", file.display(), old_value, new_value);
                Ok(true)
            } else {
                retag_file(&info, tag, &new_value, opts)
            }
        }
        None => {
            verbose!(opts, "{}: no change", file.display());
            Ok(false)
        }
    }
}

fn new_tag(old_value: &str, rx: &Regex, to: &str) -> anyhow::Result<Option<String>> {
    let new_value = rx.replace_all(old_value, to);

    if new_value == old_value {
        Ok(None)
    } else {
        Ok(Some(new_value.to_string()))
    }
}

fn retag_file(info: &AurMetadata, tag: &str, new: &str, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let tagger = Tagger::new(info)?;
    tagger.set_tag(tag, new, opts.quiet)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::{defopts, fixture};
    use assert_fs::prelude::*;

    #[test]
    fn test_new_tag() {
        assert_eq!(
            None,
            new_tag("The Tag", &Regex::new("no match").unwrap(), "sub").unwrap()
        );

        assert_eq!(
            Some("New Tag".to_string()),
            new_tag("Old Tag", &Regex::new("Old").unwrap(), "New").unwrap()
        );

        assert_eq!(
            Some("Tag the Tag".to_string()),
            new_tag("Can the Can", &Regex::new("Can").unwrap(), "Tag").unwrap()
        );

        assert_eq!(
            Some("Two Cats and a Dog".to_string()),
            new_tag(
                "Two Dogs and a Cat",
                &Regex::new("(Dog)(.*)(Cat)").unwrap(),
                "${3}${2}${1}"
            )
            .unwrap()
        );

        assert_eq!(
            Some("Nerd Nerd Nerd".to_string()),
            new_tag("Word Word Word", &Regex::new("Wo(..)").unwrap(), "Ne${1}").unwrap()
        );
    }

    #[test]
    fn test_process_file_no_change() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        let rx = Regex::new("no match").unwrap();

        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Artist", original_info.tags.artist);
        assert!(!process_file(&file_under_test, "title", &rx, "whatever", &defopts()).unwrap());
    }

    #[test]
    fn test_process_file_change() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        let rx = Regex::new("Test").unwrap();

        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let original_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Test Artist", original_info.tags.artist);

        assert!(process_file(&file_under_test, "artist", &rx, "Tested", &defopts()).unwrap());
        let new_info = AurMetadata::new(&file_under_test).unwrap();
        assert_eq!("Tested Artist", new_info.tags.artist);
    }
}
