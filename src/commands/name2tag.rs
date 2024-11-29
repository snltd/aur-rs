use crate::utils::config::{load_config, Config};
use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::tag_maker::TagMaker;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use crate::utils::words::Words;
use std::path::Path;

pub fn run(files: &[String], opts: &GlobalOpts) -> anyhow::Result<()> {
    let config = load_config(&opts.config)?;

    for f in media_files(pathbuf_set(files)) {
        tag_file(&f, &config)?
    }

    Ok(())
}

fn tag_file(file: &Path, config: &Config) -> anyhow::Result<()> {
    let info = AurMetadata::new(file)?;
    let tagger = Tagger::new(&info)?;
    let words = Words::new(config);
    let tag_maker = TagMaker::new(&words);
    let new_tags = tag_maker.all_tags_from(&info)?;

    tagger.set_artist(&new_tags.artist)?;
    tagger.set_album(&new_tags.album)?;
    tagger.set_title(&new_tags.title)?;
    tagger.set_t_num(&new_tags.t_num.to_string())?;
    Ok(())
}
