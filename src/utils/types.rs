use clap::ValueEnum;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum RenumberDirection {
    Up,
    Down,
}

pub struct CopytagsOptions {
    pub recurse: bool,
    pub force: bool,
}

#[allow(dead_code)]
pub struct GlobalOpts {
    pub verbose: bool,
    pub noop: bool,
}
