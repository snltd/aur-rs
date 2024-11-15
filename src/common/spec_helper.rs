use std::env::current_dir;
// use std::fs;
use std::path::PathBuf;

pub fn fixture(file: &str) -> PathBuf {
    current_dir().unwrap().join("tests/resources").join(file)
}

// pub fn sample_output(file: &str) -> String {
//     let file = current_dir().unwrap().join("test/outputs").join(file);
//     fs::read_to_string(file).unwrap()
// }
