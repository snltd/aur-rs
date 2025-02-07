use crate::utils::metadata::{AurMetadata, AurTags};
use anyhow::anyhow;
use id3::TagLike;
use metaflac::block::PictureType;
use std::path::PathBuf;

// A common interface to apply the tags we're interested in to FLACs and MP3s.

pub struct Tagger<'a> {
    path: &'a PathBuf,
    filetype: &'a String,
    current_tags: &'a AurTags,
}

impl<'a> Tagger<'a> {
    pub fn new(fileinfo: &'a AurMetadata) -> anyhow::Result<Self> {
        Ok(Tagger {
            path: &fileinfo.path,
            filetype: &fileinfo.filetype,
            current_tags: &fileinfo.tags,
        })
    }

    pub fn set_artist(&self, value: &str, silent: bool) -> anyhow::Result<bool> {
        self.set_tag("artist", value, silent)
    }

    pub fn set_title(&self, value: &str, silent: bool) -> anyhow::Result<bool> {
        self.set_tag("title", value, silent)
    }

    pub fn set_album(&self, value: &str, silent: bool) -> anyhow::Result<bool> {
        self.set_tag("album", value, silent)
    }

    pub fn set_t_num(&self, value: &str, silent: bool) -> anyhow::Result<bool> {
        self.set_tag("t_num", value, silent)
    }

    pub fn set_year(&self, value: &str, silent: bool) -> anyhow::Result<bool> {
        self.set_tag("year", value, silent)
    }

    pub fn set_genre(&self, value: &str, silent: bool) -> anyhow::Result<bool> {
        self.set_tag("genre", value, silent)
    }

    // True if it changed the tag, false if it didn't.
    pub fn set_tag(&self, tag_name: &str, value: &str, silent: bool) -> anyhow::Result<bool> {
        let current_value = match tag_name {
            "artist" => &self.current_tags.artist,
            "title" => &self.current_tags.title,
            "album" => &self.current_tags.album,
            "t_num" => &self.current_tags.t_num.to_string(),
            "year" => &self.current_tags.year.to_string(),
            "genre" => &self.current_tags.genre,
            _ => return Err(anyhow!("Unknown tag name")),
        };

        if current_value == value {
            return Ok(false);
        }

        if !silent {
            println!("{:>16} -> {}", tag_name, value);
        }

        match self.filetype.as_str() {
            "flac" => self.set_flac_tag(tag_name, value),
            "mp3" => self.set_mp3_tag(tag_name, value),
            _ => Err(anyhow!("Unsupported filetype")),
        }
    }

    fn set_flac_tag(&self, tag_name: &str, value: &str) -> anyhow::Result<bool> {
        let mut tag = metaflac::Tag::read_from_path(self.path)?;
        let val = vec![value];

        match tag_name {
            "artist" => tag.set_vorbis("artist".to_string(), val),
            "album" => tag.set_vorbis("album".to_string(), val),
            "title" => tag.set_vorbis("title".to_string(), val),
            "t_num" => tag.set_vorbis("tracknumber".to_string(), val),
            "year" => tag.set_vorbis("date".to_string(), val),
            "genre" => tag.set_vorbis("genre".to_string(), val),
            _ => return Err(anyhow!("unknown tag name: {tag_name}")),
        }
        tag.save()?;
        Ok(true)
    }

    fn set_mp3_tag(&self, tag_name: &str, value: &str) -> anyhow::Result<bool> {
        let mut tag = id3::Tag::read_from_path(self.path)?;

        match tag_name {
            "artist" => tag.set_artist(value),
            "album" => tag.set_album(value),
            "title" => tag.set_title(value),
            "t_num" => tag.set_track(value.to_string().parse::<u32>()?),
            "year" => tag.set_year(value.to_string().parse::<i32>()?),
            "genre" => tag.set_genre(value),
            _ => return Err(anyhow!("unknown tag name: {tag_name}")),
        }

        tag.write_to_path(self.path, id3::Version::Id3v24)?;
        Ok(true)
    }

    pub fn remove_tags(&self, tags: &Vec<String>) -> anyhow::Result<bool> {
        match self.filetype.as_str() {
            "flac" => self.remove_flac_tags(tags),
            "mp3" => self.remove_mp3_tags(tags),
            _ => Err(anyhow!("Unsupported filetype")),
        }
    }

    fn remove_flac_tags(&self, tags: &Vec<String>) -> anyhow::Result<bool> {
        let mut tagger = metaflac::Tag::read_from_path(self.path)?;
        let mut ret = false;

        for tag_name in tags {
            let values: Vec<String> = tagger
                .get_vorbis(tag_name)
                .map(|t| t.map(|v| v.to_string()).collect())
                .unwrap_or_default();

            for v in values {
                tagger.remove_vorbis_pair(tag_name, &v);
                ret = true;
            }
        }

        if ret {
            tagger.save()?;
        }
        Ok(ret)
    }

    fn remove_mp3_tags(&self, tags: &Vec<String>) -> anyhow::Result<bool> {
        let mut tag = id3::Tag::read_from_path(self.path)?;
        let mut ret = false;

        for tag_name in tags {
            if tag.get(tag_name).is_some() {
                tag.remove(tag_name);
                ret = true;
            }

            let tag_name_uc = tag_name.to_uppercase();

            if tag.get(&tag_name_uc).is_some() {
                tag.remove(&tag_name_uc);
                ret = true;
            }
        }

        if ret {
            tag.write_to_path(self.path, id3::Version::Id3v24)?;
        }

        Ok(ret)
    }

    pub fn remove_artwork(&self) -> anyhow::Result<bool> {
        match self.filetype.as_str() {
            "flac" => self.remove_flac_artwork(),
            "mp3" => self.remove_mp3_artwork(),
            _ => Err(anyhow!("Unsupported filetype")),
        }
    }

    fn remove_flac_artwork(&self) -> anyhow::Result<bool> {
        let mut tagger = metaflac::Tag::read_from_path(self.path)?;
        tagger.remove_picture_type(PictureType::CoverFront);
        tagger.remove_picture_type(PictureType::CoverBack);
        tagger.remove_picture_type(PictureType::Media);
        tagger.save()?;
        Ok(true)
    }

    fn remove_mp3_artwork(&self) -> anyhow::Result<bool> {
        let mut tag = id3::Tag::read_from_path(self.path)?;
        tag.remove_all_pictures();
        tag.write_to_path(self.path, id3::Version::Id3v24)?;
        Ok(true)
    }

    pub fn batch_tag(&self, src_tags: &AurTags, silent: bool) -> Result<bool, anyhow::Error> {
        let changes = [
            self.set_artist(&src_tags.artist, silent)?,
            self.set_title(&src_tags.title, silent)?,
            self.set_album(&src_tags.album, silent)?,
            self.set_genre(&src_tags.genre, silent)?,
            self.set_t_num(&src_tags.t_num.to_string(), silent)?,
            self.set_year(&src_tags.year.to_string(), silent)?,
        ]
        .iter()
        .any(|&changed| changed);
        Ok(changes)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::metadata::AurMetadata;
    use crate::utils::spec_helper::fixture;
    use assert_fs::prelude::*;

    #[test]
    fn test_set_artist_flac() {
        let file = "test.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file]).unwrap();
        let flac = tmp.path().join(file);
        let original_info = AurMetadata::new(&flac).unwrap();
        let tagger = Tagger::new(&original_info).unwrap();
        assert_eq!(tagger.current_tags.artist, "Test Artist".to_string());
        assert!(!tagger.set_artist("Test Artist", false).unwrap());
        assert!(tagger.set_artist("New Artist", false).unwrap());
        let new_info = AurMetadata::new(&flac).unwrap();
        assert_eq!("New Artist".to_string(), new_info.tags.artist);
    }

    #[test]
    fn test_set_album_mp3() {
        let file = "test.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file]).unwrap();
        let flac = tmp.path().join(file);
        let original_info = AurMetadata::new(&flac).unwrap();
        let tagger = Tagger::new(&original_info).unwrap();
        assert_eq!(tagger.current_tags.album, "Test Album".to_string());
        assert!(!tagger.set_album("Test Album", false).unwrap());
        assert!(tagger.set_album("New Album", false).unwrap());
        let new_info = AurMetadata::new(&flac).unwrap();
        assert_eq!("New Album".to_string(), new_info.tags.album);
    }

    #[test]
    fn test_set_title_flac() {
        let file = "test.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file]).unwrap();
        let flac = tmp.path().join(file);
        let original_info = AurMetadata::new(&flac).unwrap();
        let tagger = Tagger::new(&original_info).unwrap();
        assert_eq!(tagger.current_tags.title, "Test Title".to_string());
        assert!(!tagger.set_title("Test Title", false).unwrap());
        assert!(tagger.set_title("New Title", false).unwrap());
        let new_info = AurMetadata::new(&flac).unwrap();
        assert_eq!("New Title".to_string(), new_info.tags.title);
    }

    #[test]
    fn test_set_genre_mp3() {
        let file = "test.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file]).unwrap();
        let flac = tmp.path().join(file);
        let original_info = AurMetadata::new(&flac).unwrap();
        let tagger = Tagger::new(&original_info).unwrap();
        assert_eq!(tagger.current_tags.genre, "Test Genre".to_string());
        assert!(!tagger.set_genre("Test Genre", false).unwrap());
        assert!(tagger.set_genre("New Genre", false).unwrap());
        let new_info = AurMetadata::new(&flac).unwrap();
        assert_eq!("New Genre".to_string(), new_info.tags.genre);
    }

    #[test]
    fn test_set_year_flac() {
        let file = "test.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file]).unwrap();
        let flac = tmp.path().join(file);
        let original_info = AurMetadata::new(&flac).unwrap();
        let tagger = Tagger::new(&original_info).unwrap();
        assert_eq!(tagger.current_tags.year, 2021);
        assert!(!tagger.set_year("2021", false).unwrap());
        assert!(tagger.set_year("2001", false).unwrap());
        let new_info = AurMetadata::new(&flac).unwrap();
        assert_eq!(2001, new_info.tags.year);
    }
}
