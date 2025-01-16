use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::mp3_encoder::{transcode_cmds, transcode_file, TranscodeAction};
use crate::utils::types::{GlobalOpts, Mp3dirOpts};
use colored::Colorize;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub fn run(
    files: &[String],
    bitrate: String,
    force: bool,
    opts: &GlobalOpts,
) -> anyhow::Result<()> {
    let cmds = transcode_cmds()?;
    let transcode_opts = Mp3dirOpts {
        bitrate,
        force,
        recurse: false,
        root: PathBuf::from("/"),
        suffix: false,
    };

    for f in media_files(&pathbuf_set(files)) {
        if let Some(action) = transcode_action(&f) {
            println!("{}", f.display().to_string().bold());
            transcode_file(&action, &cmds, &transcode_opts, opts)?;
        } else {
            eprintln!("ERROR: Only FLAC files can be flac2mp3-ed");
        }
    }

    Ok(())
}

fn transcode_action(file: &Path) -> Option<TranscodeAction> {
    match file.extension() {
        Some(ext) => {
            if ext == OsStr::new("flac") {
                let mp3_target = file.with_extension("mp3");

                let action: TranscodeAction = TranscodeAction {
                    flac_src: file.to_path_buf(),
                    mp3_target,
                };

                Some(action)
            } else {
                None
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_transcode_mp3() {
        assert!(transcode_action(&fixture("commands/flac2mp3/01.tester.test_no-op.mp3")).is_none());
    }

    #[test]
    fn test_transcode_flac() {
        let flac = fixture("commands/flac2mp3/01.tester.test_transcode.flac");
        let mp3 = fixture("commands/flac2mp3/01.tester.test_transcode.mp3");

        assert_eq!(
            TranscodeAction {
                flac_src: flac.clone(),
                mp3_target: mp3,
            },
            transcode_action(&flac).unwrap()
        );
    }
}
