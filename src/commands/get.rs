use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use std::path::{Path, PathBuf};

// Get prints things "the wrong way round", because I sometimes want to run the results through
// sort(1).

type Info = (PathBuf, String);
type InfoList = Vec<Info>;

pub fn run(property: &str, files: &[String], short: bool) -> anyhow::Result<()> {
    let mut info_list: InfoList = Vec::new();

    for f in media_files(&pathbuf_set(files)) {
        let info = info_for_file(property, &f)?;
        info_list.push((f, info));
    }

    info_list
        .iter()
        .for_each(|info| print_file_info(info, short));
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
        _ => data.get_tag(property)?,
    };

    Ok(ret)
}

fn print_file_info(info: &Info, short: bool) {
    if short {
        println!("{}", info.1);
    } else {
        println!("{:>20} : {}", info.1, info.0.display());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_info_for_file() {
        assert_eq!(
            "16-bit/44100Hz",
            info_for_file("bitrate", &fixture("info/test.flac")).unwrap()
        );

        assert_eq!(
            "64kbps",
            info_for_file("bitrate", &fixture("info/test.mp3")).unwrap()
        );
    }
}
