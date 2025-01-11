use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::rename::rename;
use crate::utils::string::ToSafe;
use crate::utils::types::GlobalOpts;
use std::path::{Path, PathBuf};

pub fn run(files: &[String], opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(&pathbuf_set(files)) {
        if let Some(target) = rename_action(&f)? {
            rename((f, target), opts.noop)?;
        } else {
            println!("nothing to do for {}", f.display());
        }
    }

    Ok(())
}

fn rename_action(file: &Path) -> anyhow::Result<Option<PathBuf>> {
    let file = file.canonicalize()?;
    let info = AurMetadata::new(&file)?;
    let cwd = file.parent().expect("cannot get parent");
    let file_name = file.file_name().expect("cannot get basename");

    if info.tags.artist.is_empty() {
        println!("Cannot get artist for {}", file.display());
        return Ok(None);
    }

    if info.tags.album.is_empty() {
        println!("Cannot get album for {}", file.display());
        return Ok(None);
    }

    let target_dir = cwd.join(format!(
        "{}.{}",
        info.tags.artist.to_safe(),
        info.tags.album.to_safe()
    ));

    Ok(Some(target_dir.join(file_name)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_rename_action() {
        assert_eq!(
            fixture("commands/sort/singer.singers_album/01.singer.song.flac"),
            rename_action(&fixture("commands/sort/01.singer.song.flac"))
                .unwrap()
                .unwrap()
        );
    }
}
