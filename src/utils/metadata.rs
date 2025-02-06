use anyhow::anyhow;
use id3::Tag as Id3Tag;
use id3::TagLike;
use metaflac::Tag as FlacTag;
use mp3_metadata::{self, MP3Metadata};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

const UNDEFINED: &str = "unknown";

pub type RawTags = Vec<(String, String)>;

#[derive(Debug)]
pub struct AurMetadata {
    pub filename: String,
    pub path: PathBuf,
    pub filetype: String,
    pub tags: AurTags,
    pub time: AurTime,
    pub quality: AurQuality,
    pub rawtags: RawTags,
    pub has_picture: bool,
    pub in_tracks: bool,
}

type AurTNum = u32;
type AurYear = i32;

#[derive(Debug, PartialEq)]
pub struct AurTags {
    pub artist: String,
    pub album: String,
    pub title: String,
    pub t_num: AurTNum,
    pub year: AurYear,
    pub genre: String,
}

#[derive(Debug)]
pub struct AurQuality {
    pub bit_depth: u16,
    pub sample_rate: u32,
    pub formatted: String,
}

#[derive(Debug)]
pub struct AurTime {
    pub raw: u64,
    pub formatted: String,
}

impl AurMetadata {
    pub fn new(file: &Path) -> anyhow::Result<Self> {
        let file = file.to_path_buf().canonicalize()?;
        let tags: AurTags;
        let filetype: String;
        let quality: AurQuality;
        let time: AurTime;
        let rawtags: RawTags;
        let has_picture: bool;
        let in_tracks = in_tracks(&file);

        match file.extension().and_then(|ext| ext.to_str()) {
            Some("flac") => {
                let raw_info = FlacTag::read_from_path(&file)?;
                filetype = "flac".to_string();
                tags = AurTags::from_flac(&raw_info)?;
                quality = AurQuality::from_flac(&raw_info)?;
                time = AurTime::from_flac(&raw_info)?;
                rawtags = Self::rawtags_from_flac(&raw_info)?;
                has_picture = Self::has_picture_flac(&raw_info)?;
            }
            Some("mp3") => match mp3_metadata::read_from_file(&file) {
                Ok(metadata) => {
                    let id3tags = Id3Tag::read_from_path(&file)?;
                    filetype = "mp3".to_string();
                    tags = AurTags::from_mp3(&id3tags)?;
                    quality = AurQuality::from_mp3(&metadata)?;
                    time = AurTime::from_mp3(&metadata)?;
                    rawtags = Self::rawtags_from_mp3(&id3tags)?;
                    has_picture = Self::has_picture_mp3(&id3tags)?;
                }
                Err(e) => {
                    return Err(anyhow!(
                        "Failed to read MP3 metadata in {}: {}",
                        file.display(),
                        e
                    ))
                }
            },
            _ => return Err(anyhow!("Unsupported filetype: {}", file.display())),
        }

        let filename = match file.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => return Err(anyhow!("Unable to work out file name: {}", file.display())),
        };

        Ok(Self {
            filename,
            path: file.to_path_buf(),
            tags,
            filetype,
            time,
            quality,
            rawtags,
            has_picture,
            in_tracks,
        })
    }

    pub fn get_tag(&self, tag: &str) -> anyhow::Result<String> {
        let ret = match tag {
            "artist" => self.tags.artist.to_string(),
            "album" => self.tags.album.to_string(),
            "title" => self.tags.title.to_string(),
            "genre" => self.tags.genre.to_string(),
            "t_num" => self.tags.t_num.to_string(),
            "year" => self.tags.year.to_string(),
            _ => return Err(anyhow!("Unknown tag: {}", tag)),
        };

        Ok(ret)
    }

    fn rawtags_from_flac(raw_info: &FlacTag) -> anyhow::Result<RawTags> {
        let ret: RawTags;

        if let Some(vorbis_comments) = raw_info.vorbis_comments() {
            ret = vorbis_comments
                .comments
                .clone()
                .into_iter()
                .map(|(tag, val)| (tag.to_lowercase(), val.join(",")))
                .collect()
        } else {
            ret = Vec::new();
        }

        Ok(ret)
    }

    fn rawtags_from_mp3(id3tag: &Id3Tag) -> anyhow::Result<RawTags> {
        let ret: RawTags = id3tag
            .frames()
            .map(|frame| {
                (
                    frame.id().to_string().to_lowercase(),
                    frame.content().to_string(),
                )
            })
            .collect();

        Ok(ret)
    }

    fn has_picture_flac(raw_info: &FlacTag) -> anyhow::Result<bool> {
        let pictures: Vec<_> = raw_info.pictures().collect();
        Ok(!pictures.is_empty())
    }

    fn has_picture_mp3(id3tag: &Id3Tag) -> anyhow::Result<bool> {
        let pictures: Vec<_> = id3tag.pictures().collect();
        Ok(!pictures.is_empty())
    }
}

impl AurTags {
    fn from_flac(raw_info: &FlacTag) -> anyhow::Result<Self> {
        let comments = raw_info.vorbis_comments();

        Ok(Self {
            artist: Self::first_or_default(comments.and_then(|c| c.artist())),
            album: Self::first_or_default(comments.and_then(|c| c.album())),
            title: Self::first_or_default(comments.and_then(|c| c.title())),
            t_num: comments.and_then(|c| c.track()).unwrap_or(0),
            year: Self::first_or_default(comments.and_then(|c| c.get("DATE")))
                .parse::<i32>()
                .unwrap_or(0),
            genre: Self::first_or_default(comments.and_then(|c| c.genre())),
        })
    }

    fn from_mp3(id3tag: &Id3Tag) -> anyhow::Result<Self> {
        Ok(Self {
            artist: id3tag.artist().unwrap_or(UNDEFINED).to_string(),
            album: id3tag.album().unwrap_or(UNDEFINED).to_string(),
            title: id3tag.title().unwrap_or(UNDEFINED).to_string(),
            t_num: id3tag.track().unwrap_or(0),
            year: id3tag.year().unwrap_or(0),
            genre: id3tag.genre().unwrap_or(UNDEFINED).to_string(),
        })
    }

    fn first_or_default(option: Option<&Vec<String>>) -> String {
        option
            .and_then(|vec| vec.first())
            .unwrap_or(&UNDEFINED.to_string())
            .clone()
    }
}

impl AurQuality {
    fn from_flac(raw_info: &FlacTag) -> anyhow::Result<Self> {
        let ret = match raw_info.get_streaminfo() {
            Some(info) => Self {
                bit_depth: info.bits_per_sample as u16,
                sample_rate: info.sample_rate,
                formatted: format!("{}-bit/{}Hz", info.bits_per_sample, info.sample_rate),
            },
            None => Self {
                bit_depth: 0,
                sample_rate: 0,
                formatted: "unknown".to_string(),
            },
        };

        Ok(ret)
    }

    fn from_mp3(metadata: &MP3Metadata) -> anyhow::Result<Self> {
        let ret = match metadata.frames.first() {
            Some(frame) => Self {
                bit_depth: frame.bitrate, // Not really bit depth, but hey. Also mp3-metadata isn't very good at VBR.
                sample_rate: frame.sampling_freq as u32,
                formatted: format!("{}kbps", frame.bitrate),
            },
            None => Self {
                bit_depth: 0,
                sample_rate: 0,
                formatted: "unknown".to_string(),
            },
        };

        Ok(ret)
    }
}

impl AurTime {
    fn from_flac(raw_info: &FlacTag) -> anyhow::Result<Self> {
        let ret = match raw_info.get_streaminfo() {
            Some(info) => {
                let duration: u64 = if info.sample_rate > 0 {
                    info.total_samples / info.sample_rate as u64
                } else {
                    0
                };

                Self {
                    raw: duration,
                    formatted: Self::format_duration(&duration),
                }
            }
            None => Self {
                raw: 0,
                formatted: "unknown".to_string(),
            },
        };

        Ok(ret)
    }

    fn from_mp3(metadata: &MP3Metadata) -> anyhow::Result<Self> {
        let duration = metadata.duration.as_secs();
        Ok(Self {
            raw: duration,
            formatted: Self::format_duration(&duration),
        })
    }

    fn format_duration(seconds: &u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

impl Default for AurTags {
    fn default() -> Self {
        Self {
            artist: UNDEFINED.to_string(),
            album: UNDEFINED.to_string(),
            title: UNDEFINED.to_string(),
            t_num: 0,
            year: 0,
            genre: UNDEFINED.to_string(),
        }
    }
}

pub fn expected_tags(filetype: &str) -> anyhow::Result<HashSet<String>> {
    match filetype {
        "flac" => Ok(HashSet::from([
            "artist".into(),
            "album".into(),
            "title".into(),
            "tracknumber".into(),
            "genre".into(),
            "date".into(),
        ])),
        "mp3" => Ok(HashSet::from([
            "tpe1".into(),
            "talb".into(),
            "tit2".into(),
            "trck".into(),
            "tyer".into(),
            "tcon".into(),
        ])),
        _ => Err(anyhow!("unknown filetype")),
    }
}

pub fn irrelevant_tags(filetype: &str) -> anyhow::Result<HashSet<String>> {
    match filetype {
        "flac" => Ok(HashSet::from(["encoder".into(), "blank".into()])),
        "mp3" => Ok(HashSet::from(["tlen".into(), "tsse".into()])),
        _ => Err(anyhow!("unknown filetype")),
    }
}

pub fn in_tracks(file: &Path) -> bool {
    match file.parent() {
        Some(parent) => match parent.file_name() {
            Some(basename) => basename == OsStr::new("tracks"),
            None => false,
        },
        None => false,
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_metadata_valid_files() {
        let expected_tags = AurTags {
            artist: "Test Artist".to_string(),
            album: "Test Album".to_string(),
            title: "Test Title".to_string(),
            genre: "Test Genre".to_string(),
            t_num: 6,
            year: 2021,
        };

        let flac_result = AurMetadata::new(&fixture("info/test.flac")).unwrap();
        let mp3_result = AurMetadata::new(&fixture("info/test.mp3")).unwrap();

        assert_eq!("Test Artist", flac_result.get_tag("artist").unwrap());

        assert_eq!("6", flac_result.get_tag("t_num").unwrap());

        assert!(flac_result.get_tag("whatever").is_err());

        assert_eq!(expected_tags, flac_result.tags);
        assert_eq!("16-bit/44100Hz", flac_result.quality.formatted);
        assert_eq!("00:00:00", flac_result.time.formatted);

        assert_eq!(expected_tags, mp3_result.tags);
        assert_eq!("64kbps", mp3_result.quality.formatted);
        assert_eq!("00:00:00", mp3_result.time.formatted);
    }

    #[test]
    fn test_metadata_missing_file() {
        let result = AurMetadata::new(&PathBuf::from("/does/not/exist"));
        assert!(
            matches!(result, Err(ref e) if e.to_string() == "No such file or directory (os error 2)")
        );
    }

    #[test]
    fn test_metadata_bad_file() {
        let flac_result = AurMetadata::new(&fixture("info/bad_file.flac"));
        assert!(
            matches!(flac_result, Err(ref e) if e.to_string() == "InvalidInput: reader does not contain flac metadata")
        );
    }
}
