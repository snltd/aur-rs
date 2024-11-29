use crate::utils::config::{load_config, Config};
use crate::utils::types::GlobalOpts;
use std::env::current_dir;
use std::path::PathBuf;

pub fn fixture(file: &str) -> PathBuf {
    current_dir().unwrap().join("tests/resources").join(file)
}

pub fn sample_config() -> Config {
    load_config(&fixture("config/test.toml")).unwrap()
}

pub fn defopts() -> GlobalOpts {
    GlobalOpts {
        verbose: false,
        noop: false,
        config: fixture("config/test.toml"),
    }
}
