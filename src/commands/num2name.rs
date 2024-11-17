use crate::utils::metadata::AurMetadata;
use crate::utils::rename;
use std::path::Path;

pub fn run(files: &[String]) -> anyhow::Result<()> {
    rename::rename_files(files, rename_action)
}

pub fn rename_action(file: &Path) -> anyhow::Result<rename::RenameOption> {
    let info = AurMetadata::new(file)?;
    let tag_track_number = info.tags.t_num;
    let filename = &info.filename;

    match rename::number_from_filename(filename.as_str()) {
        Some((num_str, num_u32)) => {
            if num_u32 == tag_track_number {
                return Ok(None);
            }

            let dest_name = filename.replacen(
                num_str.as_str(),
                rename::padded_num(tag_track_number).as_str(),
                1,
            );

            let dest = info
                .path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Failed to get directory of {:?}", file))?
                .join(dest_name);

            Ok(Some((file.to_path_buf(), dest)))
        }
        None => {
            let dest_name = format!("{}.{}", rename::padded_num(tag_track_number), filename);
            let dest = info
                .path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Failed to get directory of {:?}", file))?
                .join(dest_name);

            Ok(Some((file.to_path_buf(), dest)))
        }
    }
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
    fn test_rename_action() {
        let fixture_dir = fixture("commands/name2num");

        assert_eq!(
            (
                fixture_dir.join("01.test_artist.test_title.flac"),
                fixture_dir.join("02.test_artist.test_title.flac"),
            ),
            rename_action(&fixture("commands/name2num/01.test_artist.test_title.flac"))
                .unwrap()
                .unwrap()
        );
    }
}
