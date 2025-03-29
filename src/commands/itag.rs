use crate::utils::config::load_config;
use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use crate::utils::rename;
use crate::utils::tag_validator::TagValidator;
use crate::utils::tagger::Tagger;
use crate::utils::types::{GlobalOpts, RenameOption};
use crate::utils::words::Words;
use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};
use std::io::{self, Write};

pub fn run(files: &[Utf8PathBuf], tag: &str, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let config = load_config(&opts.config)?;
    let words = Words::new(&config);
    let validator = TagValidator::new(&words, config.get_genres());
    let mut ret = true;

    for f in media_files(&pathbuf_set(files)) {
        let value = read_value(&f, tag)?;

        if !validator.validate_tag(tag, value.as_str())? {
            eprintln!("ERROR: '{}' is not a valid {} value", value, tag);
            ret = false;
            continue;
        }

        if let Some(action) = tag_and_rename_action(&f, tag, &value)? {
            rename::rename(action, opts.noop)?;
        }
    }

    Ok(ret)
}

fn read_value(file: &Utf8Path, tag: &str) -> anyhow::Result<String> {
    let basename = file.file_name().context("could not get file name")?;

    print!("{} [{}]> ", basename, tag);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn tag_and_rename_action(file: &Utf8Path, tag: &str, value: &str) -> anyhow::Result<RenameOption> {
    let info = AurMetadata::new(file)?;
    let retagged = Tagger::new(&info)?.set_tag(tag, value, true)?;

    if !retagged {
        return Ok(None);
    }

    rename::rename_action_from_file(file)
}
