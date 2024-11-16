use crate::common::metadata::AurMetadata;
use anyhow::anyhow;
use std::path::{Path, PathBuf};

// Get prints things "the wrong way round", because I sometimes want to run the results through
// sort(1).

type Info = (PathBuf, String);
type InfoList = Vec<Info>;

pub fn run(property: &str, files: &[String]) -> anyhow::Result<()> {
    let file_info: InfoList = files
        .iter()
        .map(|file| {
            let path = PathBuf::from(file);
            info_for_file(property, &path).map(|info| (path, info))
        })
        .collect::<Result<Vec<_>, _>>()?;

    file_info.iter().for_each(print_file_info);
    Ok(())
}

fn info_for_file(property: &str, file: &Path) -> anyhow::Result<String> {
    let data = AurMetadata::new(file)?;

    let ret = match property {
        "bit_depth" => data.quality.bit_depth.to_string(),
        "sample_rate" => data.quality.sample_rate.to_string(),
        "bitrate" => data.quality.formatted,
        "time" => data.time.formatted,
        "time_raw" => data.time.raw.to_string(),
        "artist" => data.tags.artist,
        "album" => data.tags.album,
        "title" => data.tags.title,
        "genre" => data.tags.genre,
        "t_num" => data.tags.t_num.to_string(),
        "year" => data.tags.year.to_string(),
        _ => return Err(anyhow!("Unknown property")),
    };

    Ok(ret)
}

fn print_file_info(info: &Info) {
    println!("{:>20} : {}", info.1, info.0.display());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::spec_helper::fixture;

    #[test]
    fn test_run_no_file() {
        if let Err(e) = run("testprop", &["/does/not/exist".to_string()]) {
            println!("{}", e);
        }
    }

    #[test]
    fn test_info_for_file() {
        assert_eq!(
            "16-bit/44100Hz".to_string(),
            info_for_file("bitrate", &fixture("info/test.flac")).unwrap()
        );

        assert_eq!(
            "64kbps".to_string(),
            info_for_file("bitrate", &fixture("info/test.mp3")).unwrap()
        );
    }
}
