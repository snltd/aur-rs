use crate::utils::metadata::AurMetadata;
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
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_run_no_file() {
        assert!(run(&["/does/not/exist".to_string()]).is_err());
    }

    #[test]
    fn test_tags() {
        let flac_result =
            info_for_file(&fixture("commands/tags/01.test_artist.test_track.flac")).unwrap();
        let mp3_result =
            info_for_file(&fixture("commands/tags/01.test_artist.test_track.mp3")).unwrap();

        assert_eq!(14, flac_result.len());
        assert_eq!("         album : Test Album".to_string(), flac_result[0]);
        assert_eq!(15, mp3_result.len());
        assert_eq!("          comm : Test Comment".to_string(), mp3_result[0]);
    }
}
