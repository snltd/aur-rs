#[allow(dead_code)]
use crate::utils::config::{load_config, Config};
use crate::utils::types::GlobalOpts;
use camino::Utf8PathBuf;
use std::env::current_dir;
use std::fs;

pub fn sample_config() -> Config {
    load_config(&fixture("config/test.toml")).unwrap()
}

pub fn defopts() -> GlobalOpts {
    GlobalOpts {
        verbose: false,
        noop: false,
        config: fixture("config/test.toml"),
        quiet: true,
    }
}

pub fn fixture(file: &str) -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(current_dir().unwrap())
        .unwrap()
        .join("tests")
        .join("resources")
        .join(file)
}

pub fn fixture_as_string(file: &str) -> String {
    fixture(file).to_string()
}

pub fn sample_output(file: &str) -> String {
    let file = current_dir().unwrap().join("tests/outputs").join(file);
    fs::read_to_string(file).unwrap()
}

pub fn config_file() -> Utf8PathBuf {
    fixture("config/test.toml")
}

pub fn config_file_as_string() -> String {
    fixture_as_string("config/test.toml")
}

#[cfg(test)]
use assert_fs::TempDir;
#[cfg(test)]
use camino::Utf8Path;

#[cfg(test)]
pub trait TempDirExt {
    fn utf8_path(&self) -> &Utf8Path;
}

#[cfg(test)]
impl TempDirExt for TempDir {
    fn utf8_path(&self) -> &Utf8Path {
        Utf8Path::from_path(self.path()).unwrap()
    }
}
