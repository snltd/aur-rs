use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tag_maker::TagMaker;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use std::path::Path;

pub fn run(files: &[String], _opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in media_files(pathbuf_set(files)) {
        tag_file(&f)?
    }

    Ok(())
}

fn tag_file(file: &Path) -> anyhow::Result<()> {
    let info = AurMetadata::new(file)?;
    let tagger = Tagger::new(&info)?;
    let new_tags = TagMaker::from_info(&info)?;
    tagger.set_artist(new_tags.artist.as_str())?;
    tagger.set_album(new_tags.album.as_str())?;
    tagger.set_title(new_tags.title.as_str())?;
    tagger.set_t_num(new_tags.t_num.to_string().as_str())?;
    Ok(())
}
