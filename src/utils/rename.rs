use crate::utils::string::ToSafe;
use anyhow::anyhow;
use std::path::{Path, PathBuf};

pub type RenameOption = Option<RenameAction>;
pub type RenameAction = (PathBuf, PathBuf);

/// Called by the command run() functions
pub fn rename_files<F>(files: &[String], action: F) -> anyhow::Result<()>
where
    F: Fn(&Path) -> anyhow::Result<RenameOption>,
{
    for file in files {
        let path = Path::new(file);

        if let Some(action) = action(path)? {
            rename(action)?;
        }
    }
    Ok(())
}

pub fn number_from_filename(fname: &str) -> Option<(String, u32)> {
    let bits = fname.split('.').collect::<Vec<&str>>();

    match bits.first() {
        Some(bit) => match bit.parse::<u32>() {
            Ok(num) => Some((bit.to_string(), num)),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn padded_num(num: u32) -> String {
    format!("{:02}", num)
}

pub fn safe_filename(num: u32, artist: &str, title: &str, filetype: &str) -> String {
    format!(
        "{}.{}.{}.{}",
        padded_num(num),
        artist.to_safe().replacen("the_", "", 1),
        title.to_safe(),
        filetype.to_lowercase()
    )
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_padded_num() {
        assert_eq!("01".to_string(), padded_num(1));
        assert_eq!("00".to_string(), padded_num(0));
        assert_eq!("76".to_string(), padded_num(76));
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
    fn test_safe_filename() {
        assert_eq!(
            "04.merpers.ive_got_something--very_loud.flac".to_string(),
            safe_filename(4, "The Merpers", "I've Got Something (Very Loud)", "FLAC")
        );

        assert_eq!(
            "23.singer.song.mp3".to_string(),
            safe_filename(23, "Singer", "SONG", "mp3")
        );
    }
}
