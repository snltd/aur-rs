use crate::utils::dir::{expand_file_list, media_files};
use crate::utils::external::find_binary;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use camino::{Utf8Path, Utf8PathBuf};
use colored::Colorize;
use rayon::prelude::*;
use std::process::Command;

struct Cmds {
    flac: Utf8PathBuf,
    mp3val: Utf8PathBuf,
}

pub fn run(files: &[Utf8PathBuf], recurse: bool, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let cmds = Cmds {
        flac: find_binary("flac")?,
        mp3val: find_binary("mp3val")?,
    };

    use std::sync::atomic::{AtomicBool, Ordering};

    let ret = AtomicBool::new(true);

    media_files(&expand_file_list(files, recurse)?)
        .par_iter()
        .for_each(|f| match verify_file(f, &cmds) {
            Ok(result) => {
                display_result(f, result, opts);
                if !result {
                    ret.store(false, Ordering::Relaxed);
                }
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", f, e);
                ret.store(false, Ordering::Relaxed);
            }
        });

    Ok(ret.load(Ordering::Relaxed))
}

fn display_result(file: &Utf8Path, result: bool, opts: &GlobalOpts) {
    if result {
        verbose!(opts, "{:^9}: {}", "OK".to_owned().green().reversed(), file);
    } else {
        println!(
            "{:^9}: {}",
            "INVALID".to_owned().bold().red().reversed(),
            file
        );
    }
}

fn verify_file(file: &Utf8Path, cmds: &Cmds) -> anyhow::Result<bool> {
    match file.extension() {
        Some(ext) => match ext {
            "flac" => verify_flac(file, &cmds.flac),
            "mp3" => verify_mp3(file, &cmds.mp3val),
            _ => Ok(false),
        },
        None => Ok(false),
    }
}

fn verify_flac(file: &Utf8Path, cmd: &Utf8Path) -> anyhow::Result<bool> {
    let result = Command::new(cmd)
        .arg("--test")
        .arg("--totally-silent")
        .arg(file)
        .status()?;

    Ok(result.success())
}

fn verify_mp3(file: &Utf8Path, cmd: &Utf8Path) -> anyhow::Result<bool> {
    // mp3val exits 0 whatever.
    let output = Command::new(cmd).arg("-si").arg(file).output()?;
    let stdout_string = String::from_utf8(output.stdout)?;

    if stdout_string.contains("WARNING") || stdout_string.contains("ERROR") {
        Ok(false)
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::fixture;

    #[test]
    fn test_verify_files() {
        let cmds = Cmds {
            flac: find_binary("flac").unwrap(),
            mp3val: find_binary("mp3val").unwrap(),
        };

        assert!(verify_file(&fixture("commands/verify/01.tester.valid.flac"), &cmds,).unwrap());
        assert!(verify_file(&fixture("commands/verify/03.tester.valid.mp3"), &cmds,).unwrap());

        assert!(
            !verify_file(&fixture("commands/verify/02.tester.truncated.flac"), &cmds,).unwrap()
        );

        assert!(!verify_file(&fixture("commands/verify/04.tester.truncated.mp3"), &cmds,).unwrap());
        assert!(!verify_file(&fixture("commands/verify/05.tester.junk.flac"), &cmds,).unwrap());
        assert!(!verify_file(&fixture("commands/verify/06.tester.junk.mp3"), &cmds,).unwrap());
    }
}
