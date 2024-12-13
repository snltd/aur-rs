use crate::utils::config::{load_config, Config};
use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::retitler::Retitler;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use crate::utils::words::Words;
use std::path::Path;

pub fn run(files: &[String], opts: &GlobalOpts) -> anyhow::Result<()> {
    let config = load_config(&opts.config)?;

    for f in media_files(&pathbuf_set(files)) {
        tag_file(&f, &config)?
    }

    Ok(())
}

fn tag_file(file: &Path, config: &Config) -> anyhow::Result<()> {
    let info = AurMetadata::new(file)?;
    let tagger = Tagger::new(&info)?;
    let words = Words::new(config);
    let rt = Retitler::new(&words);

    let artist = rt.retitle(&info.tags.artist);
    tagger.set_artist(&artist)?;

    let album = rt.retitle(&info.tags.album);
    tagger.set_album(&album)?;

    let title = rt.retitle(&info.tags.title);
    tagger.set_title(&title)?;

    let t_num = rt.retitle(&info.tags.t_num.to_string());
    tagger.set_t_num(&t_num)?;

    let genre = rt.retitle(&info.tags.genre);
    tagger.set_genre(&genre)?;

    Ok(())
}
