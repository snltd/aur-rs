use crate::utils::external::find_binary;
use crate::utils::metadata::AurMetadata;
use crate::utils::tagger::Tagger;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use anyhow::anyhow;
use colored::Colorize;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::{create_dir_all, read_dir, remove_file};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::types::Mp3dirOpts;

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
    overwrite: bool,
) -> anyhow::Result<Vec<TranscodeAction>> {
    let mut ret = Vec::new();

    for stem in file_stems(flac_dir, "flac")? {
        let mp3_target = mp3_dir.join(format!("{}.mp3", stem));

        if overwrite || !mp3_target.exists() {
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
    cmd_opts: &Mp3dirOpts,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    if action.mp3_target.exists() && !cmd_opts.force {
        println!("  target exists ({})", action.mp3_target.display());
        return Ok(false);
    }

    println!(
        "  {}",
        action.flac_src.file_name().unwrap().to_string_lossy()
    );

    if opts.noop {
        return Ok(false);
    }

    let flac_info = AurMetadata::new(&action.flac_src)?;

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
        .arg("-q1")
        .arg("--vbr-new")
        .arg("-V0")
        .arg("--preset")
        .arg("extreme")
        .arg("--add-id3v2")
        .arg("--id3v2-only")
        .arg("--silent")
        .stdin(Stdio::from(flac_stdout))
        .arg("-")
        .arg(&action.mp3_target)
        .spawn()?;

    lame_encode.wait()?;
    flac_decode.wait()?;

    // Turns out slashes are separators in ID3 tags, so LAME will drop them if we pass tag
    // values which contain them. So now we tag as a separate stage.

    let mp3_info = AurMetadata::new(&action.mp3_target)?;
    Tagger::new(&mp3_info)?.batch_tag(&flac_info.tags, !opts.verbose)
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

pub fn sync_dir(
    flac_dir: &Path,
    mp3_dir: &Path,
    cmds: &TranscodeCmds,
    cmd_opts: &Mp3dirOpts,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    if flac_dir == mp3_dir {
        return Err(anyhow!(
            "FLAC and MP3 directories have the same path: {}",
            flac_dir.display()
        ));
    }

    let list = make_transcode_list(flac_dir, mp3_dir, cmd_opts.force)?;

    if !list.is_empty() {
        println!(
            "{} -> {}",
            flac_dir.display().to_string().bold(),
            mp3_dir.display()
        );
        if !mp3_dir.exists() && !opts.noop {
            verbose!(opts, "  Creating target");
            create_dir_all(mp3_dir)?;
        }

        list.par_iter()
            .try_for_each(|t| transcode_file(t, cmds, cmd_opts, opts).map(|_| ()))?;
    }

    if mp3_dir.exists() && mp3_dir.file_name().unwrap() != "tracks" {
        // it might not be there if we just no-oped, and we allow tracks/ to be different
        let clean_up_list = make_clean_up_list(flac_dir, mp3_dir)?;

        if !clean_up_list.is_empty() {
            println!("{}", mp3_dir.display().to_string().bold());
        }

        for f in clean_up_list {
            clean_up_file(&f, opts)?;
        }
    }

    Ok(true)
}

pub fn clean_up_file(superfluous_mp3: &Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    verbose!(opts, "  Removing {}", superfluous_mp3.display());

    if opts.noop {
        Ok(false)
    } else {
        remove_file(superfluous_mp3)?;
        Ok(true)
    }
}

pub fn mp3_dir_from(flac_dir: &Path, cmd_opts: &Mp3dirOpts) -> PathBuf {
    let mut path = flac_dir
        .to_string_lossy()
        .to_string()
        .replace("/flac", "/mp3");

    if cmd_opts.suffix {
        path.push_str(&format!("-{}", cmd_opts.bitrate));
    }

    PathBuf::from(path)
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

        let cmd_opts = Mp3dirOpts {
            bitrate: "320".to_string(),
            force: false,
            recurse: false,
            root: PathBuf::from("/storage"),
            suffix: false,
        };

        assert!(!mp3_file.exists());
        assert!(transcode_file(&action, &cmds, &cmd_opts, &defopts()).unwrap());
        assert!(mp3_file.exists());
        let flac_info = AurMetadata::new(&file_under_test).unwrap();
        let mp3_info = AurMetadata::new(&mp3_file).unwrap();

        assert_eq!(&flac_info.tags, &mp3_info.tags);
        assert_eq!(&flac_info.time.raw, &mp3_info.time.raw);
        assert!(mp3_info.quality.bit_depth >= 128);
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
                false,
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
                false,
            )
            .unwrap()
        );
    }
    #[test]
    fn test_mp3_dir_from() {
        assert_eq!(
            PathBuf::from("/storage/mp3/tracks"),
            mp3_dir_from(
                &PathBuf::from("/storage/flac/tracks"),
                &Mp3dirOpts {
                    bitrate: "128".into(),
                    force: false,
                    recurse: false,
                    root: "/storage".into(),
                    suffix: false
                }
            ),
        );

        assert_eq!(
            PathBuf::from("/storage/mp3/eps/band.ep-128"),
            mp3_dir_from(
                &PathBuf::from("/storage/flac/eps/band.ep"),
                &Mp3dirOpts {
                    bitrate: "128".into(),
                    force: false,
                    recurse: false,
                    root: "/storage".into(),
                    suffix: true
                }
            ),
        );
    }
}
