use crate::utils::tagger::Tagger;
use crate::utils::{metadata::AurMetadata, rename};

use super::types::GlobalOpts;

pub fn update_file(info: &AurMetadata, number: u32, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let ret_tag = tag_file(info, number, opts)?;
    let new_info = AurMetadata::new(&info.path)?;

    match rename::renumber_file(&new_info)? {
        Some(action) => {
            let ret_rename = rename::rename(action, false)?;
            Ok(ret_tag || ret_rename)
        }
        None => Ok(ret_tag),
    }
}

fn tag_file(info: &AurMetadata, number: u32, opts: &GlobalOpts) -> anyhow::Result<bool> {
    if info.tags.t_num == number {
        return Ok(false);
    }

    let tagger = Tagger::new(info)?;
    tagger.set_t_num(&number.to_string(), opts.quiet)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::{defopts, fixture};
    use assert_fs::prelude::*;

    #[test]
    fn test_update_file_change_both() {
        let start_file_name = "13.change_both.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        let start_file = tmp.path().join(start_file_name);
        tmp.copy_from(fixture("commands/inumber"), &[start_file_name])
            .unwrap();
        let start_info = AurMetadata::new(&start_file).unwrap();

        assert!(update_file(&start_info, 9, &defopts()).unwrap());

        let final_file_name = "09.change_both.mp3";
        let final_file = tmp.path().join(final_file_name);
        assert!(final_file.exists());
        let final_info = AurMetadata::new(&final_file).unwrap();
        assert_eq!(9, final_info.tags.t_num);

        // Nothing should happen this time

        assert!(!update_file(&final_info, 9, &defopts()).unwrap());
    }

    #[test]
    fn test_update_file_change_name() {
        let start_file_name = "03.change_name.mp3";
        let final_file_name = "06.change_name.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        let start_file = tmp.path().join(start_file_name);
        tmp.copy_from(fixture("commands/inumber"), &[start_file_name])
            .unwrap();
        let start_info = AurMetadata::new(&start_file).unwrap();

        assert!(update_file(&start_info, 6, &defopts()).unwrap());

        let final_file = tmp.path().join(final_file_name);
        assert!(final_file.exists());
        let final_info = AurMetadata::new(&final_file).unwrap();
        assert_eq!(6, final_info.tags.t_num);

        // Nothing should happen this time

        assert!(!update_file(&final_info, 6, &defopts()).unwrap());
    }

    #[test]
    fn test_update_file_change_tag() {
        let start_file_name = "01.change_tag.mp3";
        let final_file_name = start_file_name;
        let tmp = assert_fs::TempDir::new().unwrap();
        let start_file = tmp.path().join(start_file_name);
        tmp.copy_from(fixture("commands/inumber"), &[start_file_name])
            .unwrap();
        let start_info = AurMetadata::new(&start_file).unwrap();

        assert!(update_file(&start_info, 1, &defopts()).unwrap());

        let final_file = tmp.path().join(final_file_name);
        assert!(final_file.exists());
        let final_info = AurMetadata::new(&final_file).unwrap();
        assert_eq!(1, final_info.tags.t_num);

        // Nothing should happen this time

        assert!(!update_file(&final_info, 1, &defopts()).unwrap());
    }

    #[test]
    fn test_update_file_change_neither() {
        let start_file_name = "02.change_neither.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        let start_file = tmp.path().join(start_file_name);
        tmp.copy_from(fixture("commands/inumber"), &[start_file_name])
            .unwrap();
        let start_info = AurMetadata::new(&start_file).unwrap();
        assert!(!update_file(&start_info, 2, &defopts()).unwrap());
        assert!(start_file.exists());
    }
}
