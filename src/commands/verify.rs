use crate::utils::dir::{expand_file_list, media_files};
use crate::utils::types::GlobalOpts;
use std::path::Path;

pub fn run(files: &[String], recurse: bool, opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(&expand_file_list(files, recurse)?) {
        verify_file(&f, opts);
    }
    Ok(())
}

fn verify_file(file: &Path, opts: &GlobalOpts) {}
