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
