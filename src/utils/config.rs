use crate::utils::types::{Genres, WantsList};
use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub const MAX_ARTWORK_SIZE: u32 = 750;
pub const MIN_ARTWORK_SIZE: u32 = 350;

type TagList = HashSet<String>;
type WordList = HashSet<String>;

#[derive(Deserialize, Debug)]
pub struct Config {
    ignore: Option<Ignore>,
    words: Option<Words>,
    genres: Option<Genres>,
}

#[derive(Deserialize, Debug)]
pub struct Ignore {
    lint: Option<LintErrs>,
    lintdir: Option<LintDirErrs>,
    syncflac: Option<HashSet<String>>,
    wantflac: Option<WantFlac>,
}

#[derive(Deserialize, Debug)]
pub struct WantFlac {
    albums: Option<WantsList>,
    top_level: Option<WantsList>,
    tracks: Option<WantsList>,
}

#[derive(Deserialize, Debug)]
pub struct Words {
    pub all_caps: Option<WordList>,
    pub expand: Option<HashMap<String, String>>,
    pub ignore_case: Option<WordList>,
    pub no_caps: Option<WordList>,
}

#[derive(Deserialize, Debug)]
pub struct LintErrs {
    pub invalid_album_tag: Option<TagList>,
    pub invalid_artist_tag: Option<TagList>,
    pub invalid_title_tag: Option<TagList>,
    pub invalid_year_tag: Option<TagList>,
}

#[derive(Deserialize, Debug)]
pub struct LintDirErrs {
    pub bad_file_count: Option<HashSet<String>>,
    pub inconsistent_tags: Option<HashSet<String>>,
}

fn home_dir() -> Utf8PathBuf {
    Utf8PathBuf::from(std::env::var("HOME").expect("cannot find home directory"))
}

pub fn default_location() -> Utf8PathBuf {
    home_dir().join(".aur.toml")
}

pub fn default_linkdir() -> Utf8PathBuf {
    home_dir().join("work").join("artfix")
}

// If the user specifies a file and it doesn't exist, that's an error. If they don't, and the
// default file doesn't exist, that's fine, and we return an empty config.
//
pub fn load_config(file: &Utf8Path) -> anyhow::Result<Config> {
    if !file.exists() && file == default_location() {
        toml::from_str("").map_err(|e| anyhow::anyhow!(e))
    } else if !file.exists() {
        Err(anyhow!(format!("Cannot find config at {}", file)))
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

    pub fn get_words_all_caps(&self) -> Option<&HashSet<String>> {
        self.words
            .as_ref()
            .and_then(|words| words.all_caps.as_ref())
    }

    pub fn get_words_no_caps(&self) -> Option<&WordList> {
        self.words.as_ref().and_then(|words| words.no_caps.as_ref())
    }

    pub fn get_words_expand(&self) -> Option<&HashMap<String, String>> {
        self.words.as_ref().and_then(|words| words.expand.as_ref())
    }

    pub fn get_words_ignore_case(&self) -> Option<&HashSet<String>> {
        self.words
            .as_ref()
            .and_then(|words| words.ignore_case.as_ref())
    }

    pub fn get_syncflac_list(&self) -> Option<&HashSet<String>> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.syncflac.as_ref())
    }

    pub fn get_ignore_lint_invalid_album(&self) -> Option<&HashSet<String>> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.lint.as_ref())
            .and_then(|lint| lint.invalid_album_tag.as_ref())
    }

    pub fn get_ignore_lint_invalid_title(&self) -> Option<&HashSet<String>> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.lint.as_ref())
            .and_then(|lint| lint.invalid_title_tag.as_ref())
    }

    pub fn get_ignore_lint_invalid_artist(&self) -> Option<&HashSet<String>> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.lint.as_ref())
            .and_then(|lint| lint.invalid_artist_tag.as_ref())
    }

    pub fn get_ignore_lint_invalid_year(&self) -> Option<&HashSet<String>> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.lint.as_ref())
            .and_then(|lint| lint.invalid_year_tag.as_ref())
    }

    pub fn get_ignore_lintdir_bad_file_count(&self) -> Option<&HashSet<String>> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.lintdir.as_ref())
            .and_then(|lintdir| lintdir.bad_file_count.as_ref())
    }

    pub fn get_ignore_lintdir_inconsistent_tags(&self) -> Option<&HashSet<String>> {
        self.ignore
            .as_ref()
            .and_then(|ignore| ignore.lintdir.as_ref())
            .and_then(|lintdir| lintdir.inconsistent_tags.as_ref())
    }

    pub fn get_genres(&self) -> Option<&Genres> {
        self.genres.as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::{fixture, sample_config};
    use std::collections::BTreeSet;

    #[test]
    fn test_load_config() {
        assert!(load_config(&fixture("config/no-such-file.toml")).is_err());
    }

    #[test]
    fn test_wantflac() {
        let config = sample_config();
        assert_eq!(
            &BTreeSet::from(["singer.song".to_string()]),
            config.get_wantflac_ignore_tracks().unwrap()
        );

        assert_eq!(
            &BTreeSet::from(["albums/abc/artist.album".to_string()]),
            config.get_wantflac_ignore_albums().unwrap()
        );

        let no_config = load_config(&fixture("config/empty.toml")).unwrap();
        assert_eq!(None, no_config.get_wantflac_ignore_tracks());
    }

    #[test]
    fn test_words() {
        let config = sample_config();
        assert_eq!(
            &HashSet::from(["mxbx".to_string()]),
            config.get_words_ignore_case().unwrap()
        );

        assert_eq!(
            &HashSet::from(["4ad".to_string(), "abba".to_string()]),
            config.get_words_all_caps().unwrap()
        );

        assert_eq!(
            &HashMap::from([("add_n_to_x".to_string(), "Add N to (X)".to_string())]),
            config.get_words_expand().unwrap()
        );

        assert_eq!(None, config.get_words_no_caps());
    }

    #[test]
    fn test_ignore_lint() {
        let config = sample_config();
        assert_eq!(
            &HashSet::from(["The R&B of Membership".to_string()]),
            config.get_ignore_lint_invalid_album().unwrap()
        );

        assert_eq!(None, config.get_ignore_lint_invalid_artist());
    }

    #[test]
    fn test_get_genres() {
        let config = sample_config();
        assert_eq!(
            &HashSet::from([
                "Alternative".to_string(),
                "Indie".to_string(),
                "Noise".to_string(),
                "Test".to_string()
            ]),
            config.get_genres().unwrap()
        );
    }
}
