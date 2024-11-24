use anyhow::anyhow;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Ignore {
    wantflac: WantFlac,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct WantFlac {
    tracks: Vec<String>,
    albums: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Config {
    ignore: Ignore,
}

#[allow(dead_code)]
pub fn load_config(file: &Path) -> anyhow::Result<Config> {
    if !file.exists() {
        return Err(anyhow!(format!("Cannot find config at {}", file.display())));
    }

    let raw = read_to_string(file)?;
    toml::from_str(&raw).map_err(|e| anyhow::anyhow!(e))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_config() {
        let config = load_config(&fixture("config/test.toml")).unwrap();

        assert_eq!(vec!["artist.album"], config.ignore.wantflac.albums);
    }
}
