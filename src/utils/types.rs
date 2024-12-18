use clap::ValueEnum;
use std::collections::HashSet;
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
    pub verbose: bool,
    pub noop: bool,
    pub config: PathBuf,
}

pub type WantsList = HashSet<String>;
