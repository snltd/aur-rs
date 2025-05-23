use crate::utils::metadata::AurMetadata;
use crate::utils::string::ToFilenameChunk;
use crate::utils::types::{RenameAction, RenameOption};
use anyhow::anyhow;
use camino::Utf8Path;
use std::fs;

pub fn rename_action_from_file(file: &Utf8Path) -> anyhow::Result<RenameOption> {
    let info = AurMetadata::new(file)?;
    rename_action_from_metadata(&info)
}

pub fn rename_action_from_metadata(info: &AurMetadata) -> anyhow::Result<RenameOption> {
    let tags = &info.tags;
    let correct_filename = safe_filename(
        tags.t_num,
        &tags.artist,
        &tags.title,
        &info.filetype,
        info.in_tracks,
    );

    if info.filename == correct_filename {
        Ok(None)
    } else {
        let dest = info
            .path
            .parent()
            .ok_or_else(|| anyhow!("Failed to get directory of {:?}", info.filename))?
            .join(correct_filename);

        Ok(Some((info.path.clone(), dest.to_path_buf())))
    }
}

pub fn number_from_filename(fname: &str) -> Option<(String, u32)> {
    let bits = fname.split(['.', ' ', '_', '-']).collect::<Vec<&str>>();

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

pub fn safe_filename(
    num: u32,
    artist: &str,
    title: &str,
    filetype: &str,
    in_tracks: bool,
) -> String {
    let mut artist = artist.to_filename_chunk();

    if artist.starts_with("the_") {
        artist = artist.replacen("the_", "", 1);
    }

    if in_tracks {
        format!(
            "{}.{}.{}",
            artist,
            title.to_filename_chunk(),
            filetype.to_lowercase()
        )
    } else {
        format!(
            "{}.{}.{}.{}",
            padded_num(num),
            artist,
            title.to_filename_chunk(),
            filetype.to_lowercase()
        )
    }
}

pub fn rename((src, dest): RenameAction, noop: bool) -> anyhow::Result<bool> {
    if src == dest {
        Ok(false)
    } else if dest.exists() && !noop {
        Err(anyhow!(format!("destination exists: {}", dest)))
    } else {
        if let Some(parent_dir) = dest.parent() {
            if !parent_dir.exists() {
                println!("Creating {}", parent_dir);
                if !noop {
                    fs::create_dir_all(parent_dir)?;
                }
            }
        }

        let src_dir = src.parent().expect("Cannot find parent of src_dir");
        let dest_dir = dest.parent().expect("Cannot find parent of dest_dir");

        let target_to_print = if dest_dir == src_dir || src_dir.as_str() == "" {
            dest.file_name().unwrap().to_owned()
        } else {
            match dest_dir.strip_prefix(src_dir) {
                Ok(relative_path) => relative_path.to_string(),
                Err(_) => dest.file_name().unwrap().to_owned(),
            }
        };

        println!("  {} -> {}", src.file_name().unwrap(), target_to_print);

        if !noop {
            fs::rename(src, dest).map_err(|e| anyhow::anyhow!(e))?;
        }
        Ok(true)
    }
}

// Makes the file number match the tag number
pub fn renumber_file(info: &AurMetadata) -> anyhow::Result<RenameOption> {
    let filename = info.filename.as_str();
    let tag_track_number = info.tags.t_num;

    let dest_name = match number_from_filename(filename) {
        Some((num_str, num_u32)) => {
            if num_u32 == tag_track_number {
                return Ok(None);
            }

            filename.replacen(&num_str, &padded_num(tag_track_number), 1)
        }
        None => {
            format!("{}.{}", padded_num(tag_track_number), filename)
        }
    };

    let dest = info
        .path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Failed to get directory of {:?}", info.path))?
        .join(dest_name);

    Ok(Some((info.path.to_owned(), dest)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::fixture;

    #[test]
    fn test_renumber_file() {
        let info = AurMetadata::new(&fixture("rename/test.mp3")).unwrap();
        assert_eq!(
            (fixture("rename/test.mp3"), fixture("rename/06.test.mp3")),
            renumber_file(&info).unwrap().unwrap(),
        );
    }

    #[test]
    fn test_padded_num() {
        assert_eq!("01", padded_num(1));
        assert_eq!("00", padded_num(0));
        assert_eq!("76", padded_num(76));
    }

    #[test]
    fn test_number_from_filename() {
        assert_eq!(
            ("03".to_owned(), 3),
            number_from_filename("03.singer.song.flac").unwrap()
        );
        assert_eq!(
            ("99".to_owned(), 99),
            number_from_filename("99.singer.song.flac").unwrap()
        );
        assert_eq!(None, number_from_filename("singer.song.flac"));
        assert_eq!(None, number_from_filename(".0a.singer.song.flac"));
    }

    #[test]
    fn test_safe_filename() {
        assert_eq!(
            "04.merpers.ive_got_something--very_loud.flac",
            safe_filename(
                4,
                "The Merpers",
                "I've Got Something (Very Loud)",
                "FLAC",
                false
            )
        );

        assert_eq!(
            "03.big_merp_and_the_merpers.merping.mp3",
            safe_filename(3, "Big Merp and The Merpers", "Merping!", "mp3", false)
        );

        assert_eq!(
            "23.singer.song.mp3",
            safe_filename(23, "Singer", "SONG", "mp3", false)
        );

        assert_eq!(
            "singer.song.mp3",
            safe_filename(23, "Singer", "SONG", "mp3", true)
        );
    }

    #[test]
    fn test_rename_action_from_file() {
        assert_eq!(
            (
                fixture("commands/tag2name/badly_named_file.mp3"),
                fixture("commands/tag2name/01.tester.some_song--or_other.mp3")
            ),
            rename_action_from_file(&fixture("commands/tag2name/badly_named_file.mp3"))
                .unwrap()
                .unwrap()
        );
    }
}
