use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use camino::{Utf8Path, Utf8PathBuf};

// Get prints things "the wrong way round", because I sometimes want to run the results through
// sort(1).

type Info = (Utf8PathBuf, String);
type InfoList = Vec<Info>;

pub fn run(property: &str, files: &[Utf8PathBuf], short: bool) -> anyhow::Result<()> {
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

fn info_for_file(property: &str, file: &Utf8Path) -> anyhow::Result<String> {
    let data = AurMetadata::new(file)?;
    let quality = data.quality();
    let time = data.time();

    let ret = match property {
        "bit_depth" => quality.bit_depth.to_string(),
        "sample_rate" => quality.sample_rate.to_string(),
        "bitrate" => quality.formatted,
        "time" => time.formatted,
        "time_raw" => time.raw.to_string(),
        _ => data.get_tag(property)?,
    };

    Ok(ret)
}

fn print_file_info(info: &Info, short: bool) {
    if short {
        println!("{}", info.1);
    } else {
        println!("{:>20} : {}", info.1, info.0);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::fixture;

    #[test]
    fn test_get_bitrate() {
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
