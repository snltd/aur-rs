use crate::common::info;
use std::path::{Path, PathBuf};

pub fn run(files: &[String]) -> anyhow::Result<()> {
    let file_info: Vec<_> = files
        .iter()
        .map(|file| info_for_file(&PathBuf::from(file)))
        .collect::<Result<_, _>>()?;

    file_info.iter().for_each(|info| print_file_info(info));
    Ok(())
}

fn info_for_file(file: &Path) -> anyhow::Result<Vec<String>> {
    let data = info::AurMetadata::new(file)?;
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
    use crate::common::spec_helper::fixture;

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
        assert_eq!(" Filename : test.flac".to_string(), flac_info[0]);
        assert_eq!("     Type : FLAC".to_string(), flac_info[1]);
        assert_eq!("  Bitrate : 16-bit/44100Hz".to_string(), flac_info[2]);
        assert_eq!("     Time : 00:00:00".to_string(), flac_info[3]);
        assert_eq!("   Artist : Test Artist".to_string(), flac_info[4]);
        assert_eq!("    Album : Test Album".to_string(), flac_info[5]);
        assert_eq!("    Title : Test Title".to_string(), flac_info[6]);
        assert_eq!("    Genre : Test Genre".to_string(), flac_info[7]);
        assert_eq!(" Track no : 6".to_string(), flac_info[8]);
        assert_eq!("     Year : 2021".to_string(), flac_info[9]);

        let mp3_info = info_for_file(&fixture("info/test.mp3")).unwrap();

        assert_eq!(10, mp3_info.len());
        assert_eq!(" Filename : test.mp3".to_string(), mp3_info[0]);
        assert_eq!("     Type : MP3".to_string(), mp3_info[1]);
        assert_eq!("  Bitrate : 64kbps".to_string(), mp3_info[2]);
        assert_eq!("     Time : 00:00:00".to_string(), mp3_info[3]);
        assert_eq!("   Artist : Test Artist".to_string(), mp3_info[4]);
        assert_eq!("    Album : Test Album".to_string(), mp3_info[5]);
        assert_eq!("    Title : Test Title".to_string(), mp3_info[6]);
        assert_eq!("    Genre : Test Genre".to_string(), mp3_info[7]);
        assert_eq!(" Track no : 6".to_string(), mp3_info[8]);
        assert_eq!("     Year : 2021".to_string(), mp3_info[9]);
    }
}
