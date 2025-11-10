use camino::Utf8PathBuf;
use clap::ValueEnum;
use std::collections::{BTreeSet, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RenumberDirection {
    Up,
    Down,
}

#[derive(Default)]
pub struct CopytagsOptions {
    pub recurse: bool,
    pub force: bool,
}

#[derive(Default)]
pub struct TranscodeOptions {
    pub force: bool,
    pub remove_originals: bool,
}

#[derive(Default)]
pub struct GlobalOpts {
    pub config: Utf8PathBuf,
    pub noop: bool,
    pub quiet: bool,
    pub verbose: bool,
}

#[derive(Default)]
pub struct Mp3dirOpts {
    pub preset: String,
    pub force: bool,
    pub recurse: bool,
    pub root: Utf8PathBuf,
    pub suffix: bool,
}

pub type WantsList = BTreeSet<String>;
pub type RenameAction = (Utf8PathBuf, Utf8PathBuf);
pub type RenameOption = Option<RenameAction>;
pub type Genres = HashSet<String>;
