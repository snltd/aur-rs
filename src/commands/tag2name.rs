use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::rename;
use crate::utils::types::{GlobalOpts, RenameOption};
use std::path::Path;

pub fn run(files: &[String], opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(&pathbuf_set(files)) {
        if let Some(action) = rename_action(&f)? {
            rename::rename(action, opts.noop)?;
        }
    }

    Ok(())
}

fn rename_action(file: &Path) -> anyhow::Result<RenameOption> {
    let info = AurMetadata::new(file)?;
    let tags = &info.tags;
    let correct_filename =
        rename::safe_filename(tags.t_num, &tags.artist, &tags.title, &info.filetype);

    if info.filename == correct_filename {
        Ok(None)
    } else {
        let dest = info
            .path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get directory of {:?}", file))?
            .join(correct_filename);

        Ok(Some((file.to_path_buf(), dest.to_path_buf())))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_rename_action() {
        assert_eq!(
            (
                fixture("commands/tag2name/badly_named_file.mp3"),
                fixture("commands/tag2name/01.tester.some_song--or_other.mp3")
            ),
            rename_action(&fixture("commands/tag2name/badly_named_file.mp3"))
                .unwrap()
                .unwrap()
        );
    }
}
