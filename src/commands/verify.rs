use crate::utils::dir::{expand_file_list, media_files};
use crate::utils::external::find_binary;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use colored::Colorize;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

struct Cmds {
    flac: PathBuf,
    mp3val: PathBuf,
}

pub fn run(files: &[String], recurse: bool, opts: &GlobalOpts) -> anyhow::Result<()> {
    let cmds = Cmds {
        flac: find_binary("flac")?,
        mp3val: find_binary("mp3val")?,
    };

    // let mut ret = true;

    for f in media_files(&expand_file_list(files, recurse)?) {
        let result = verify_file(&f, &cmds)?;

        display_result(&f, result, opts);

        // if !result {
        //     ret = false
        // }
    }

    Ok(())
}

fn display_result(file: &Path, result: bool, opts: &GlobalOpts) {
    if result {
        verbose!(
            opts,
            "{:^9}: {}",
            "OK".to_string().green().reversed(),
            file.display()
        );
    } else {
        println!(
            "{:^9}: {}",
            "INVALID".to_string().bold().red().reversed(),
            file.display()
        );
    }
}

fn verify_file(file: &Path, cmds: &Cmds) -> anyhow::Result<bool> {
    match file.extension() {
        Some(ext) => {
            if ext == OsStr::new("flac") {
                verify_flac(file, &cmds.flac)
            } else if ext == OsStr::new("mp3") {
                verify_mp3(file, &cmds.mp3val)
            } else {
                Ok(false)
            }
        }
        None => Ok(false),
    }
}

fn verify_flac(file: &Path, cmd: &PathBuf) -> anyhow::Result<bool> {
    let result = Command::new(cmd)
        .arg("--test")
        .arg("--totally-silent")
        .arg(file)
        .status()?;

    Ok(result.success())
}

fn verify_mp3(file: &Path, cmd: &PathBuf) -> anyhow::Result<bool> {
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
    use crate::utils::spec_helper::fixture;

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
