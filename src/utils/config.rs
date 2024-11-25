use crate::utils::types::WantsList;
use anyhow::anyhow;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Config {
    ignore: Option<Ignore>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Ignore {
    wantflac: Option<WantFlac>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct WantFlac {
    tracks: Option<WantsList>,
    albums: Option<WantsList>,
    top_level: Option<WantsList>,
}

pub fn default_location() -> String {
    let home = std::env::var("HOME").expect("cannot find home directory");
    let home_dir = PathBuf::from(home);
    home_dir.join(".aur.toml").to_string_lossy().to_string()
}

// If the user specifies a file and it doesn't exist, that's an error. If they don't, and the
// default file doesn't exist, that's fine, and we return an empty config.
pub fn load_config(file: &Path) -> anyhow::Result<Config> {
    if !file.exists() && file == PathBuf::from(default_location()) {
        toml::from_str("").map_err(|e| anyhow::anyhow!(e))
    } else if !file.exists() {
        Err(anyhow!(format!("Cannot find config at {}", file.display())))
    } else {
        let raw = read_to_string(file)?;
        toml::from_str(&raw).map_err(|e| anyhow::anyhow!(e))
    }
}

impl Config {
    pub fn get_wantflac_ignore_tracks(&self) -> Option<&WantsList> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.wantflac.as_ref())
            .and_then(|wantflac| wantflac.tracks.as_ref())
    }

    pub fn get_wantflac_ignore_albums(&self) -> Option<&WantsList> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.wantflac.as_ref())
            .and_then(|wantflac| wantflac.albums.as_ref())
    }

    pub fn get_wantflac_ignore_top_level(&self) -> Option<&WantsList> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.wantflac.as_ref())
            .and_then(|wantflac| wantflac.top_level.as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;
    use std::collections::HashSet;

    #[test]
    fn test_load_config() {
        assert!(load_config(&fixture("config/no-such-file.toml")).is_err());
    }

    #[test]
    fn test_wantflac() {
        let config = load_config(&fixture("config/test.toml")).unwrap();
        assert_eq!(
            &HashSet::from(["singer.song".to_string()]),
            config
                .get_wantflac_ignore_tracks()
                .unwrap_or(&HashSet::new())
        );

        assert_eq!(
            &HashSet::from(["albums/abc/artist.album".to_string()]),
            config
                .get_wantflac_ignore_albums()
                .unwrap_or(&HashSet::new())
        );

        let no_config = load_config(&fixture("config/empty.toml")).unwrap();

        assert_eq!(
            &HashSet::new(),
            no_config
                .get_wantflac_ignore_tracks()
                .unwrap_or(&HashSet::new())
        );
    }
}
