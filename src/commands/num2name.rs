use crate::common::metadata::AurMetadata;
use anyhow::anyhow;
use std::path::{Path, PathBuf};

type RenameOption = Option<RenameAction>;
type RenameAction = (PathBuf, PathBuf);

pub fn run(files: &[String]) -> anyhow::Result<()> {
    for file in files {
        match rename_action(&PathBuf::from(file))? {
            Some(action) => rename(action)?,
            None => continue, // Skip to the next file if there's no action
        }
    }
    Ok(())
}

fn number_from_filename(fname: &str) -> Option<(String, u32)> {
    let bits = fname.split('.').collect::<Vec<&str>>();

    match bits.first() {
        Some(bit) => match bit.parse::<u32>() {
            Ok(num) => Some((bit.to_string(), num)),
            Err(_) => None,
        },
        None => None,
    }
}

fn padded_num(num: u32) -> String {
    format!("{:02}", num)
}

fn rename((src, dest): RenameAction) -> anyhow::Result<()> {
    if dest.exists() {
        Err(anyhow!(format!("destination exists: {}", dest.display())))
    } else {
        println!(
            "  {} -> {}",
            src.file_name().unwrap().to_string_lossy(),
            dest.file_name().unwrap().to_string_lossy(),
        );
        std::fs::rename(src, dest).map_err(|e| anyhow::anyhow!(e))
    }
}

fn rename_action(file: &Path) -> anyhow::Result<RenameOption> {
    let info = AurMetadata::new(file)?;
    let tag_track_number = info.tags.t_num;

    let dest_name: String;

    if let Some((num_str, num_u32)) = number_from_filename(info.filename.as_str()) {
        // file has a number already
        if num_u32 == tag_track_number {
            return Ok(None);
        } else {
            dest_name =
                info.filename
                    .replacen(num_str.as_str(), padded_num(tag_track_number).as_str(), 1);
        }
    } else {
        // File does not have a leadnign number so give it the tag.
        dest_name = format!("{}.{}", padded_num(tag_track_number), info.filename);
    }

    let dest = info.path.parent().unwrap().join(dest_name);

    Ok(Some((file.to_path_buf(), dest.to_path_buf())))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::spec_helper::fixture;

    #[test]
    fn test_run_no_file() {
        if let Err(e) = run(&["/does/not/exist".to_string()]) {
            println!("{}", e);
        }
    }

    #[test]
    fn test_number_from_filename() {
        assert_eq!(
            ("03".to_string(), 3),
            number_from_filename("03.singer.song.flac").unwrap()
        );
        assert_eq!(
            ("99".to_string(), 99),
            number_from_filename("99.singer.song.flac").unwrap()
        );
        assert_eq!(None, number_from_filename("singer.song.flac"));
        assert_eq!(None, number_from_filename(".0a.singer.song.flac"));
    }

    #[test]
    fn test_padded_num() {
        assert_eq!("01".to_string(), padded_num(1));
        assert_eq!("00".to_string(), padded_num(0));
        assert_eq!("76".to_string(), padded_num(76));
    }

    #[test]
    fn test_rename_action() {
        let fixture_dir = fixture("commands/name2num");

        assert_eq!(
            (
                fixture_dir.join("01.test_artist.test_title.flac"),
                fixture_dir.join("02.test_artist.test_title.flac"),
            ),
            rename_action(&fixture("commands/name2num/01.test_artist.test_title.flac"))
                .unwrap()
                .unwrap()
        );
    }
}
