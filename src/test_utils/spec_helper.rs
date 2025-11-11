use crate::utils::config::{Config, load_config};
use crate::utils::types::GlobalOpts;
use camino::Utf8PathBuf;
use std::env::current_dir;
use std::fs;

#[allow(dead_code)]
pub fn sample_config() -> Config {
    load_config(&fixture("config/test.toml")).unwrap()
}

#[allow(dead_code)]
pub fn defopts() -> GlobalOpts {
    GlobalOpts {
        verbose: false,
        noop: false,
        config: fixture("config/test.toml"),
        quiet: true,
    }
}

#[allow(dead_code)]
pub fn fixture(file: &str) -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(current_dir().unwrap())
        .unwrap()
        .join("tests")
        .join("resources")
        .join(file)
}

#[allow(dead_code)]
pub fn fixture_as_string(file: &str) -> String {
    fixture(file).to_string()
}

#[allow(dead_code)]
pub fn sample_output(file: &str) -> String {
    let file = current_dir().unwrap().join("tests/outputs").join(file);
    fs::read_to_string(file).unwrap()
}

#[allow(dead_code)]
pub fn config_file() -> Utf8PathBuf {
    fixture("config/test.toml")
}

#[allow(dead_code)]
pub fn config_file_as_string() -> String {
    fixture_as_string("config/test.toml")
}
