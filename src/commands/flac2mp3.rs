use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::lame_wrapper::{transcode_cmds, transcode_file, TranscodeAction};
use crate::utils::types::GlobalOpts;
use std::ffi::OsStr;
use std::path::Path;

pub fn run(files: &[String], opts: &GlobalOpts) -> anyhow::Result<()> {
    let cmds = transcode_cmds()?;

    // The transcoding output, which was written before this command, doesn't give us quite what
    // we want, so I'm going to force some options to suppress the default output, and print our
    // own.

    let forced_opts = GlobalOpts {
        quiet: true,
        verbose: false,
        config: opts.config.clone(),
        noop: opts.noop,
    };

    for f in media_files(&pathbuf_set(files)) {
        if let Some(target) = transcode_target(&f) {
            println!("{} -> {}", f.display(), target.mp3_target.display());
            transcode_file(&target, &cmds, &forced_opts)?;
        } else {
            eprintln!("ERROR: Only FLAC files can be flac2mp3-ed");
        }
    }

    Ok(())
}

fn transcode_target(file: &Path) -> Option<TranscodeAction> {
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
        assert!(transcode_target(&fixture("commands/flac2mp3/01.tester.test_no-op.mp3")).is_none());
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
            transcode_target(&flac).unwrap()
        );
    }
}
