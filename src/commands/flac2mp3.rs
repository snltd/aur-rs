use crate::utils::dir;
use crate::utils::mp3_encoder::{self, TranscodeAction};
use crate::utils::types::{GlobalOpts, Mp3dirOpts};
use camino::{Utf8Path, Utf8PathBuf};
use colored::Colorize;

pub fn run(
    files: &[Utf8PathBuf],
    preset: String,
    force: bool,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let cmds = mp3_encoder::transcode_cmds()?;
    let transcode_opts = Mp3dirOpts {
        preset,
        force,
        recurse: false,
        root: Utf8PathBuf::from("/"),
        suffix: false,
    };

    let mut ret = true;

    for f in dir::media_files(&dir::pathbuf_set(files)) {
        if let Some(action) = transcode_action(&f) {
            println!("{}", f.to_string().bold());
            if !mp3_encoder::transcode_file(&action, &cmds, &transcode_opts, opts)? {
                ret = false;
            }
        } else {
            eprintln!("ERROR: Only FLAC files can be flac2mp3-ed");
            ret = false;
        }
    }

    Ok(ret)
}

fn transcode_action(file: &Utf8Path) -> Option<TranscodeAction> {
    match file.extension() {
        Some(ext) => {
            if ext == "flac" {
                let mp3_target = file.with_extension("mp3");

                let action = TranscodeAction {
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
    use crate::test_utils::spec_helper::fixture;

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
