use crate::utils::dir;
use crate::utils::external::find_binary;
use crate::utils::metadata::AurMetadata;
use crate::utils::string::ReplaceLast;
use crate::utils::types::GlobalOpts;
use crate::{err_if_empty, verbose};
use anyhow::ensure;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs::rename;
use std::process::Command;

pub fn run(
    files: &[Utf8PathBuf],
    leave_originals: bool,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let ffmpeg = find_binary("ffmpeg")?;
    let mut ret_code = true;
    let files = dir::media_files(&dir::pathbuf_set(files));
    err_if_empty!(files);

    for file in files {
        if let Err(e) = reencode_file(&file, leave_originals, &ffmpeg, opts) {
            eprintln!("Error reencoding {file}: {e}");
            ret_code = false;
        }
    }

    Ok(ret_code)
}

fn reencode_file(
    file: &Utf8Path,
    leave_original: bool,
    ffmpeg: &Utf8Path,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let info = AurMetadata::new(file)?;

    ensure!(info.filetype == "flac", "Only FLAC files can be CDQed");

    if info.quality.bit_depth == 16 && info.quality.sample_rate == 44100 {
        verbose!(opts, "{}: already 16/44100", file);
        return Ok(false);
    }

    let work_dir = file.parent().unwrap();
    let output_file = work_dir.join(cdq_name(&info.filename));

    verbose!(opts, "{} -> {}", file, output_file);

    Command::new(ffmpeg)
        .arg("-hide_banner")
        .arg("-loglevel")
        .arg("error")
        .arg("-i")
        .arg(file)
        .arg("-af")
        .arg("aresample=out_sample_fmt=s16:out_sample_rate=44100")
        .arg(&output_file)
        .output()?;

    if leave_original {
        Ok(true)
    } else {
        rename(&output_file, file)?;
        Ok(true)
    }
}

fn cdq_name(original_name: &str) -> String {
    original_name.replace_last(".flac", "-cdq.flac")
}

#[cfg(test)]
mod test {
    use super::*;
    use camino_tempfile_ext::prelude::*;
    use snltest::fixture;

    #[test]
    fn test_cdq_name() {
        assert_eq!("01.artist.song-cdq.flac", cdq_name("01.artist.song.flac"));
    }

    #[test]
    fn test_cdq_reencode_overwrite() {
        let ffmpeg = find_binary("ffmpeg").unwrap();
        let leave_original = false;
        let file_name = "01.tester.hi-res.flac";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/cdq"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let cdq_file = tmp.path().join("01.tester.hi-res-cdq.flac");
        let original_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("24-bit/96000Hz", original_info.quality.formatted);
        assert!(reencode_file(&file_under_test, leave_original, &ffmpeg, &GlobalOpts::default()).unwrap());

        let new_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("16-bit/44100Hz", new_info.quality.formatted);
        assert!(!reencode_file(&file_under_test, leave_original, &ffmpeg, &GlobalOpts::default()).unwrap());
        assert!(!cdq_file.exists());
    }

    #[test]
    fn test_cdq_reencode_leave_original() {
        let ffmpeg = find_binary("ffmpeg").unwrap();
        let leave_original = true;
        let file_name = "01.tester.hi-res.flac";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/cdq"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let cdq_file = tmp.path().join("01.tester.hi-res-cdq.flac");
        let original_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("24-bit/96000Hz", original_info.quality.formatted);
        assert!(reencode_file(&file_under_test, leave_original, &ffmpeg, &GlobalOpts::default()).unwrap());

        let new_original_info = AurMetadata::new(&file_under_test).unwrap();

        assert_eq!("24-bit/96000Hz", new_original_info.quality.formatted);

        let new_info = AurMetadata::new(&cdq_file).unwrap();

        assert_eq!("16-bit/44100Hz", new_info.quality.formatted);
        assert!(cdq_file.exists());
    }

    #[test]
    fn test_cdq_mp3() {
        let ffmpeg = find_binary("ffmpeg").unwrap();
        let file_name = "02.tester.not_a_flac.mp3";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/cdq"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert!(reencode_file(&file_under_test, true, &ffmpeg, &GlobalOpts::default()).is_err());
    }
}
