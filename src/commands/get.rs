use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use std::path::{Path, PathBuf};

// Get prints things "the wrong way round", because I sometimes want to run the results through
// sort(1).

type Info = (PathBuf, String);
type InfoList = Vec<Info>;

pub fn run(property: &str, files: &[String]) -> anyhow::Result<()> {
    let mut info_list: InfoList = Vec::new();

    for f in media_files(pathbuf_set(files)) {
        let info = info_for_file(property, &f)?;
        info_list.push((f, info));
    }

    info_list.iter().for_each(print_file_info);
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

fn print_file_info(info: &Info) {
    println!("{:>20} : {}", info.1, info.0.display());
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

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
