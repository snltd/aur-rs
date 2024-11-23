use crate::utils::dir::{expand_file_list, media_files};
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use crate::utils::types::{CopytagsOptions, GlobalOpts};
use crate::verbose;
use anyhow::anyhow;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

pub fn run(
    files: &[String],
    ct_opts: &CopytagsOptions,
    global_opts: &GlobalOpts,
) -> anyhow::Result<()> {
    for f in media_files(expand_file_list(files, ct_opts.recurse)?) {
        tag_file(&f, ct_opts, global_opts)?;
    }
    Ok(())
}

fn tag_file(file: &Path, ct_opts: &CopytagsOptions, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;
    let partner_path = match find_partner(&info, ct_opts.force)? {
        Some(file) => {
            verbose!(opts, "Partner is {}", file.display());
            file
        }
        None => {
            return Err(anyhow!(
                "{} has no partner from which to copy tags",
                file.display()
            ))
        }
    };

    let file_tags = &info.tags;
    let partner_info = AurMetadata::new(&partner_path)?;
    let partner_tags = &partner_info.tags;

    if file_tags == partner_tags {
        verbose!(opts, "Tags already match");
        return Ok(false);
    }

    let tagger = Tagger::new(&info)?;

    let changes = [
        tagger.set_artist(partner_tags.artist.as_str())?,
        tagger.set_title(partner_tags.title.as_str())?,
        tagger.set_album(partner_tags.album.as_str())?,
        tagger.set_genre(partner_tags.genre.as_str())?,
        tagger.set_t_num(partner_tags.t_num.to_string().as_str())?,
        tagger.set_year(partner_tags.year.to_string().as_str())?,
    ]
    .iter()
    .any(|&changed| changed);

    Ok(changes)
}

fn find_partner(info: &AurMetadata, force: bool) -> anyhow::Result<Option<PathBuf>> {
    let filetype = info.filetype.as_str();
    let filename_str = info.filename.as_str();

    let newtype = match filetype {
        "mp3" => "flac",
        "flac" => "mp3",
        _ => return Err(anyhow!(format!("unknown filetype: {}", filetype))),
    };

    let partner_path = info
        .path
        .components()
        .map(|c| {
            if c.as_os_str() == OsStr::new(filetype) {
                OsString::from(newtype)
            } else if c.as_os_str() == OsStr::new(filename_str) {
                let modified_filename = info.filename.replace(filetype, newtype);
                OsString::from(modified_filename)
            } else {
                OsString::from(c.as_os_str())
            }
        })
        .collect::<PathBuf>();

    // Unless the user set the force option, we won't offer a partner that is older than file

    if partner_path.exists() {
        if force {
            Ok(Some(partner_path))
        } else {
            let partner_mtime = fs::metadata(&partner_path)?.mtime();
            let source_mtime = fs::metadata(&info.path)?.mtime();

            if partner_mtime > source_mtime {
                Ok(Some(partner_path))
            } else {
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_find_partner() {
        assert_eq!(
            fixture("commands/copytags/flac/01.artist.song.flac"),
            find_partner(
                &AurMetadata::new(&fixture("commands/copytags/mp3/01.artist.song.mp3")).unwrap(),
                true,
            )
            .unwrap()
            .unwrap(),
        );

        assert_eq!(
            fixture("commands/copytags/mp3/01.artist.song.mp3"),
            find_partner(
                &AurMetadata::new(&fixture("commands/copytags/flac/01.artist.song.flac")).unwrap(),
                true,
            )
            .unwrap()
            .unwrap(),
        );

        assert_eq!(
            None,
            find_partner(
                &AurMetadata::new(&fixture("commands/copytags/mp3/02.artist.song.mp3")).unwrap(),
                true,
            )
            .unwrap(),
        );
    }
}
