use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use camino::{Utf8Path, Utf8PathBuf};
use std::collections::HashMap;

type InfoMap = HashMap<Utf8PathBuf, Vec<(String, String)>>;

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<bool> {
    let mut info_set: InfoMap = HashMap::new();

    for f in media_files(&pathbuf_set(files)) {
        info_set.insert(f.clone(), info_for_file(&f)?);
    }

    info_set
        .iter()
        .for_each(|(path, info)| print_file_info(path, info));
    Ok(true)
}

fn info_for_file(file: &Utf8Path) -> anyhow::Result<Vec<(String, String)>> {
    let data = AurMetadata::new(file)?;
    let mut tags = data.rawtags;
    tags.sort();
    Ok(tags.into_iter().collect())
}

fn print_file_info(path: &Utf8PathBuf, info: &[(String, String)]) {
    println!("{}", path);
    for (k, v) in info {
        println!("{:>14} : {}", k, v);
    }
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
        assert_eq!(
            ("album".to_owned(), "Test Album".to_owned()),
            flac_result[0]
        );
        assert_eq!(15, mp3_result.len());
        assert_eq!(
            ("comm".to_owned(), "Test Comment".to_owned()),
            mp3_result[0]
        );
    }
}
