use crate::utils::metadata::AurMetadata;
use crate::utils::rename::rename;
use crate::utils::string::ToFilenameChunk;
use crate::utils::types::GlobalOpts;
use crate::{err_if_empty, utils::dir};
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(files: &[Utf8PathBuf], opts: &GlobalOpts) -> anyhow::Result<bool> {
    let mut ret_code = false;
    let files = dir::media_files(&dir::pathbuf_set(files));
    err_if_empty!(files);

    for f in files {
        match rename_action(&f) {
            Ok(target_opt) => {
                if let Some(target) = target_opt {
                    rename((f, target), opts.noop)?;
                } else {
                    println!("nothing to do for {}", f);
                }
            }
            Err(e) => {
                eprintln!("Error renaming {f}: {e}");
                ret_code = false;
            }
        }
    }

    Ok(ret_code)
}

fn rename_action(file: &Utf8Path) -> anyhow::Result<Option<Utf8PathBuf>> {
    let file = file.canonicalize_utf8()?;
    let info = AurMetadata::new(&file)?;
    let cwd = file.parent().expect("cannot get parent");
    let file_name = file.file_name().expect("cannot get basename");

    if info.tags.artist.is_empty() {
        println!("Cannot get artist for {}", file);
        return Ok(None);
    }

    if info.tags.album.is_empty() {
        println!("Cannot get album for {}", file);
        return Ok(None);
    }

    let target_dir = cwd.join(format!(
        "{}.{}",
        info.tags.artist.to_filename_chunk(),
        info.tags.album.to_filename_chunk()
    ));

    Ok(Some(target_dir.join(file_name)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::fixture;

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
