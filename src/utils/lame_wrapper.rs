use crate::utils::external::find_binary;
use crate::utils::metadata::AurMetadata;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use anyhow::anyhow;
use std::collections::HashSet;
use std::fs::{read_dir, remove_file};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct TranscodeAction {
    pub flac_src: PathBuf,
    pub mp3_target: PathBuf,
}

pub type CleanupAction = PathBuf;

pub struct TranscodeCmds {
    flac: PathBuf,
    lame: PathBuf,
}

pub fn transcode_cmds() -> anyhow::Result<TranscodeCmds> {
    Ok(TranscodeCmds {
        lame: find_binary("lame")?,
        flac: find_binary("flac")?,
    })
}

pub fn make_transcode_list(
    flac_dir: &Path,
    mp3_dir: &Path,
) -> anyhow::Result<Vec<TranscodeAction>> {
    let mut ret = Vec::new();

    for stem in file_stems(flac_dir, "flac")? {
        let mp3_target = mp3_dir.join(format!("{}.mp3", stem));

        if !mp3_target.exists() {
            ret.push(TranscodeAction {
                flac_src: flac_dir.join(format!("{}.flac", stem)),
                mp3_target,
            });
        }
    }

    Ok(ret)
}

pub fn transcode_file(
    action: &TranscodeAction,
    cmds: &TranscodeCmds,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    if action.mp3_target.exists() {
        verbose!(opts, "target '{}' exists", action.mp3_target.display());
        return Ok(false);
    }

    verbose!(
        opts,
        "Transcoding {} -> {}",
        action.flac_src.display(),
        action.mp3_target.display()
    );

    if opts.noop {
        return Ok(false);
    }

    let info = AurMetadata::new(&action.flac_src)?;

    println!(
        "{} -> {}",
        &action.flac_src.display(),
        &action.mp3_target.display()
    );

    let mut flac_decode = Command::new(&cmds.flac)
        .arg("-dsc")
        .arg(&action.flac_src)
        .stdout(Stdio::piped())
        .spawn()?;

    let flac_stdout = flac_decode
        .stdout
        .take()
        .ok_or_else(|| anyhow!("Failed to decode flac stdout"))?;

    let mut lame_encode = Command::new(&cmds.lame)
        .arg("-q2")
        .arg("--vbr-new")
        .arg("--preset")
        .arg("128")
        .arg("--add-id3v2")
        .arg("--id3v2-only")
        .arg("--silent")
        .arg("--tt")
        .arg(info.tags.title)
        .arg("--ta")
        .arg(info.tags.artist)
        .arg("--tl")
        .arg(info.tags.album)
        .arg("--ty")
        .arg(info.tags.year.to_string())
        .arg("--tn")
        .arg(info.tags.t_num.to_string())
        .arg("--tg")
        .arg(info.tags.genre)
        .stdin(Stdio::from(flac_stdout))
        .arg("-")
        .arg(&action.mp3_target)
        .spawn()?;

    lame_encode.wait()?;
    flac_decode.wait()?;
    Ok(true)
}

fn file_stems(dir: &Path, suffix: &str) -> anyhow::Result<HashSet<String>> {
    Ok(read_dir(dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_string_lossy() == suffix {
                path.file_stem()?.to_string_lossy().to_string().into()
            } else {
                None
            }
        })
        .collect())
}

pub fn make_clean_up_list(flac_dir: &Path, mp3_dir: &Path) -> anyhow::Result<Vec<CleanupAction>> {
    let mut ret = Vec::new();

    for stem in file_stems(mp3_dir, "mp3")? {
        let mp3_file = mp3_dir.join(format!("{}.mp3", stem));
        let flac_source = flac_dir.join(format!("{}.flac", stem));
        if !flac_source.exists() {
            ret.push(mp3_file);
        }
    }

    Ok(ret)
}

pub fn clean_up_file(superfluous_mp3: &Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    verbose!(opts, "Removing {}", superfluous_mp3.display());

    if opts.noop {
        Ok(false)
    } else {
        remove_file(superfluous_mp3)?;
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::{defopts, fixture};
    use assert_fs::prelude::*;
    use assert_unordered::assert_eq_unordered;

    #[test]
    fn test_transcode_file() {
        let cmds = TranscodeCmds {
            lame: find_binary("lame").unwrap(),
            flac: find_binary("flac").unwrap(),
        };

        let file_name = "02.band.song_2.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(
            fixture("commands/syncflac/flac/eps/band.flac_ep"),
            &[file_name],
        )
        .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let mp3_file = tmp.path().join("02.band.song_2.mp3");

        let action = TranscodeAction {
            flac_src: file_under_test.clone(),
            mp3_target: mp3_file.clone(),
        };

        assert!(!mp3_file.exists());
        assert!(transcode_file(&action, &cmds, &defopts()).unwrap());
        assert!(mp3_file.exists());
        let flac_info = AurMetadata::new(&file_under_test).unwrap();
        let mp3_info = AurMetadata::new(&mp3_file).unwrap();

        assert_eq!(&flac_info.tags, &mp3_info.tags);
        assert_eq!(&flac_info.time.raw, &mp3_info.time.raw);
        assert!(128 >= mp3_info.quality.bit_depth);
    }

    #[test]
    fn test_cleanup_list() {
        assert_eq!(
            vec![fixture(
                "commands/syncflac/mp3/eps/band.flac_and_mp3_unequal/03.band.song_3.mp3"
            )],
            make_clean_up_list(
                &fixture("commands/syncflac/flac/eps/band.flac_and_mp3_unequal"),
                &fixture("commands/syncflac/mp3/eps/band.flac_and_mp3_unequal"),
            )
            .unwrap()
        );

        assert_eq!(
            Vec::<CleanupAction>::new(),
            make_clean_up_list(
                &fixture("commands/syncflac/flac/albums/abc/already.synced"),
                &fixture("commands/syncflac/mp3/albums/abc/already.synced"),
            )
            .unwrap()
        );
    }

    #[test]
    fn test_transcode_list() {
        assert_eq!(
            Vec::<TranscodeAction>::new(),
            make_transcode_list(
                &fixture("commands/syncflac/flac/albums/abc/already.synced"),
                &fixture("commands/syncflac/mp3/albums/abc/already.synced"),
            )
            .unwrap()
        );

        assert_eq_unordered!(
            vec![
                TranscodeAction {
                    flac_src: fixture(
                        "commands/syncflac/flac/albums/tuv/tester.flac_album/01.tester.song_1.flac"
                    ),
                    mp3_target: fixture(
                        "commands/syncflac/mp3/albums/tuv/tester.flac_album/01.tester.song_1.mp3"
                    ),
                },
                TranscodeAction {
                    flac_src: fixture(
                        "commands/syncflac/flac/albums/tuv/tester.flac_album/02.tester.song_2.flac"
                    ),
                    mp3_target: fixture(
                        "commands/syncflac/mp3/albums/tuv/tester.flac_album/02.tester.song_2.mp3"
                    ),
                }
            ],
            make_transcode_list(
                &fixture("commands/syncflac/flac/albums/tuv/tester.flac_album"),
                &fixture("commands/syncflac/mp3/albums/tuv/tester.flac_album"),
            )
            .unwrap()
        );
    }
}
