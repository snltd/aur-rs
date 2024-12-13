use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use std::path::Path;

pub fn run(files: &[String]) -> anyhow::Result<()> {
    let files = media_files(&pathbuf_set(files));
    let mut info_list: Vec<Vec<String>> = Vec::new();

    for f in files {
        info_list.push(info_for_file(&f)?);
    }

    info_list.iter().for_each(|info| print_file_info(info));
    Ok(())
}

fn info_for_file(file: &Path) -> anyhow::Result<Vec<String>> {
    let data = AurMetadata::new(file)?;
    let fields: Vec<(&str, String)> = vec![
        ("Filename", data.filename),
        ("Type", data.filetype.to_uppercase()),
        ("Bitrate", data.quality.formatted),
        ("Time", data.time.formatted),
        ("Artist", data.tags.artist),
        ("Album", data.tags.album),
        ("Title", data.tags.title),
        ("Genre", data.tags.genre),
        ("Track no", data.tags.t_num.to_string()),
        ("Year", data.tags.year.to_string()),
    ];

    Ok(fields
        .iter()
        .map(|(key, val)| format!("{:>9} : {}", key, val))
        .collect())
}

fn print_file_info(info: &[String]) {
    println!("{}", info.join("\n"));
    println!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_run_no_file() {
        if let Err(e) = run(&["/does/not/exist".to_string()]) {
            println!("{}", e);
        }
    }

    #[test]
    fn test_info_for_file() {
        let flac_info = info_for_file(&fixture("info/test.flac")).unwrap();

        assert_eq!(10, flac_info.len());
        assert_eq!(" Filename : test.flac", flac_info[0]);
        assert_eq!("     Type : FLAC", flac_info[1]);
        assert_eq!("  Bitrate : 16-bit/44100Hz", flac_info[2]);
        assert_eq!("     Time : 00:00:00", flac_info[3]);
        assert_eq!("   Artist : Test Artist", flac_info[4]);
        assert_eq!("    Album : Test Album", flac_info[5]);
        assert_eq!("    Title : Test Title", flac_info[6]);
        assert_eq!("    Genre : Test Genre", flac_info[7]);
        assert_eq!(" Track no : 6", flac_info[8]);
        assert_eq!("     Year : 2021", flac_info[9]);

        let mp3_info = info_for_file(&fixture("info/test.mp3")).unwrap();

        assert_eq!(10, mp3_info.len());
        assert_eq!(" Filename : test.mp3", mp3_info[0]);
        assert_eq!("     Type : MP3", mp3_info[1]);
        assert_eq!("  Bitrate : 64kbps", mp3_info[2]);
        assert_eq!("     Time : 00:00:00", mp3_info[3]);
        assert_eq!("   Artist : Test Artist", mp3_info[4]);
        assert_eq!("    Album : Test Album", mp3_info[5]);
        assert_eq!("    Title : Test Title", mp3_info[6]);
        assert_eq!("    Genre : Test Genre", mp3_info[7]);
        assert_eq!(" Track no : 6", mp3_info[8]);
        assert_eq!("     Year : 2021", mp3_info[9]);
    }
}
