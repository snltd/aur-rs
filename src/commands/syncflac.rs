use crate::utils::config::{load_config, Config};
use crate::utils::dir::expand_dir_list;
use crate::utils::external::find_binary;
use crate::utils::helpers::check_hierarchy;
use crate::utils::metadata::AurMetadata;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use anyhow::anyhow;
use std::collections::HashSet;
use std::fs::{create_dir_all, read_dir, remove_file};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct TranscodeAction {
    flac_src: PathBuf,
    mp3_target: PathBuf,
}

type CleanupAction = PathBuf;

struct Cmds {
    flac: PathBuf,
    lame: PathBuf,
}

pub fn run(root_dir: &str, opts: &GlobalOpts) -> anyhow::Result<()> {
    let cmds = Cmds {
        lame: find_binary("lame")?,
        flac: find_binary("flac")?,
    };
    let root = PathBuf::from(root_dir).canonicalize()?;
    let conf = load_config(&opts.config)?;

    check_hierarchy(&root)?;
    syncflac(root.join("flac"), &conf, &cmds, opts)?;
    Ok(())
}

fn syncflac(
    flac_root: PathBuf,
    conf: &Config,
    cmds: &Cmds,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let mut dir_list = expand_dir_list(&[flac_root], true);

    if let Some(ignore_list) = conf.get_syncflac_list() {
        dir_list.retain(|d| {
            !ignore_list
                .iter()
                .any(|s| d.to_string_lossy().to_string().starts_with(s))
        });
    }

    let mut synced = 0;

    for flac_dir in dir_list.iter() {
        let mp3_dir = mp3_dir_from(flac_dir);
        if sync_dir(flac_dir, &mp3_dir, cmds, opts)? {
            synced += 1;
        }
    }

    Ok(synced > 0)
}

fn sync_dir(
    flac_dir: &Path,
    mp3_dir: &Path,
    cmds: &Cmds,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    if flac_dir == mp3_dir {
        return Err(anyhow!(
            "FLAC and MP3 directories have the same path: {}",
            flac_dir.display()
        ));
    }

    if !mp3_dir.exists() && !opts.noop {
        verbose!(opts, "Creating directory {}", mp3_dir.display());
        create_dir_all(mp3_dir)?;
    }

    for t in transcode_list(flac_dir, mp3_dir)? {
        transcode_file(&t, cmds, opts)?;
    }

    if mp3_dir.exists() && mp3_dir.file_name().unwrap() != "tracks" {
        // it might not be there if we just no-oped, and we allow tracks/ to be different
        for f in cleanup_list(flac_dir, mp3_dir)? {
            clean_up_file(&f, opts)?;
        }
    }

    Ok(true)
}

fn transcode_list(flac_dir: &Path, mp3_dir: &Path) -> anyhow::Result<Vec<TranscodeAction>> {
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

fn cleanup_list(flac_dir: &Path, mp3_dir: &Path) -> anyhow::Result<Vec<CleanupAction>> {
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

fn transcode_file(
    action: &TranscodeAction,
    cmds: &Cmds,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
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

    let flac_decode = Command::new(&cmds.flac)
        .arg("-dsc")
        .arg(&action.flac_src)
        .stdout(Stdio::piped())
        .spawn()?;

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
        .stdin(flac_decode.stdout.unwrap())
        .arg("-")
        .arg(&action.mp3_target)
        .spawn()?;

    lame_encode.wait()?;
    Ok(true)
}

fn clean_up_file(superfluous_mp3: &Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    verbose!(opts, "Removing {}", superfluous_mp3.display());

    if opts.noop {
        Ok(false)
    } else {
        remove_file(superfluous_mp3)?;
        Ok(true)
    }
}

fn mp3_dir_from(flac_dir: &Path) -> PathBuf {
    PathBuf::from(
        flac_dir
            .to_string_lossy()
            .to_string()
            .replace("/flac", "/mp3"),
    )
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::{defopts, fixture};
    use assert_fs::prelude::*;
    use assert_unordered::assert_eq_unordered;

    #[test]
    fn test_transcode_file() {
        let cmds = Cmds {
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
    fn test_transcode_list() {
        assert_eq!(
            Vec::<TranscodeAction>::new(),
            transcode_list(
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
            transcode_list(
                &fixture("commands/syncflac/flac/albums/tuv/tester.flac_album"),
                &fixture("commands/syncflac/mp3/albums/tuv/tester.flac_album"),
            )
            .unwrap()
        );
    }

    #[test]
    fn test_cleanup_list() {
        assert_eq!(
            vec![fixture(
                "commands/syncflac/mp3/eps/band.flac_and_mp3_unequal/03.band.song_3.mp3"
            )],
            cleanup_list(
                &fixture("commands/syncflac/flac/eps/band.flac_and_mp3_unequal"),
                &fixture("commands/syncflac/mp3/eps/band.flac_and_mp3_unequal"),
            )
            .unwrap()
        );

        assert_eq!(
            Vec::<CleanupAction>::new(),
            cleanup_list(
                &fixture("commands/syncflac/flac/albums/abc/already.synced"),
                &fixture("commands/syncflac/mp3/albums/abc/already.synced"),
            )
            .unwrap()
        );
    }

    #[test]
    fn test_mp3_dir_from() {
        assert_eq!(
            PathBuf::from("/storage/mp3/tracks"),
            mp3_dir_from(&PathBuf::from("/storage/flac/tracks")),
        );
    }
}
