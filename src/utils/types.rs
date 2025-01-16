use clap::ValueEnum;
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RenumberDirection {
    Up,
    Down,
}

pub struct CopytagsOptions {
    pub recurse: bool,
    pub force: bool,
}

pub struct TranscodeOptions {
    pub force: bool,
    pub remove_originals: bool,
}

pub struct GlobalOpts {
    pub config: PathBuf,
    pub noop: bool,
    pub quiet: bool,
    pub verbose: bool,
}

pub struct Mp3dirOpts {
    pub bitrate: String,
    pub force: bool,
    pub recurse: bool,
    pub suffix: bool,
}

pub type WantsList = BTreeSet<String>;
pub type RenameAction = (PathBuf, PathBuf);
pub type RenameOption = Option<RenameAction>;
