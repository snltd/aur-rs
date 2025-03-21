use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<()> {
    let mut info_list: Vec<Vec<String>> = Vec::new();

    for f in media_files(&pathbuf_set(files)) {
        info_list.push(info_for_file(&f)?);
    }

    info_list.iter().for_each(|info| print_file_info(info));
    Ok(())
}

fn info_for_file(file: &Utf8Path) -> anyhow::Result<Vec<String>> {
    let data = AurMetadata::new(file)?;
    let mut tags = data.rawtags;
    tags.sort();
    Ok(tags
        .iter()
        .map(|(k, v)| format!("{:>14} : {}", k, v))
        .collect())
}

fn print_file_info(info: &[String]) {
    println!("{}", info.join("\n"));
    println!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::fixture;

    #[test]
    fn test_tags() {
        let flac_result =
            info_for_file(&fixture("commands/tags/01.test_artist.test_track.flac")).unwrap();
        let mp3_result =
            info_for_file(&fixture("commands/tags/01.test_artist.test_track.mp3")).unwrap();

        assert_eq!(14, flac_result.len());
        assert_eq!("         album : Test Album", flac_result[0]);
        assert_eq!(15, mp3_result.len());
        assert_eq!("          comm : Test Comment", mp3_result[0]);
    }
}
