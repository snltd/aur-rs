use crate::utils::config::MAX_ARTWORK_SIZE;
use crate::utils::dir::expand_dir_list;
use crate::utils::types::GlobalOpts;
use crate::verbose;
use camino::{Utf8Path, Utf8PathBuf};
use image::imageops::FilterType::Lanczos3;
use image::{GenericImageView, ImageReader};
use indicatif::ProgressBar;
use std::fs;
use std::os::unix::fs::symlink;

pub fn run(
    dirlist: &[Utf8PathBuf],
    recurse: bool,
    linkdir: Utf8PathBuf,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let dirs = expand_dir_list(dirlist, recurse);
    let bar = ProgressBar::new(dirs.len() as u64);

    for dir in dirs {
        bar.inc(1);
        check_artwork(&dir, &linkdir, opts)?;
    }

    bar.finish();
    Ok(true)
}

fn check_artwork(dir: &Utf8Path, linkdir: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let expected_artwork = dir.join("front.jpg");

    let mut changes: Vec<bool> = Vec::new();

    if !expected_artwork.exists() {
        changes.push(rename(dir, &expected_artwork, opts)?);
    }

    if expected_artwork.exists() {
        changes.push(resize_or_link(&expected_artwork, linkdir, opts)?);
    }

    if changes.iter().any(|c| *c) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn rename(dir: &Utf8Path, front: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let mut ret = false;

    // Yes, this will flatten multiple files into one. That's fine.
    for file in jpgs_in(dir)? {
        if file != front {
            if !opts.quiet {
                println!("Rename: {} -> front.jpg", file);
            }
            if !opts.noop {
                fs::rename(file, front)?;
            }
            ret = true;
        }
    }

    Ok(ret)
}

// It's not our job to flag up problems, only to fix what we can. Lintdir will point out what
// we can't fix.
fn resize_or_link(file: &Utf8Path, linkdir: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    let img = ImageReader::open(file)?.decode()?;
    let (x, y) = img.dimensions();

    if x != y {
        return symlink_art(file, linkdir, opts);
    }

    if x <= MAX_ARTWORK_SIZE {
        return Ok(false);
    }

    if !opts.quiet {
        println!("Resize: {} -> {s}x{s}", file, s = MAX_ARTWORK_SIZE);
    }

    if !opts.noop {
        let new_img = img.resize_exact(MAX_ARTWORK_SIZE, MAX_ARTWORK_SIZE, Lanczos3);
        new_img.save(file)?;
    }

    Ok(true)
}

fn target_filename(file: &Utf8Path) -> String {
    file.to_string()
        .replace('/', "-")
        .trim_start_matches('-')
        .into()
}

fn symlink_art(file: &Utf8Path, linkdir: &Utf8Path, opts: &GlobalOpts) -> anyhow::Result<bool> {
    if !linkdir.exists() {
        verbose!(opts, "Creating artfix dir: {}", linkdir);
        if !opts.noop {
            fs::create_dir_all(linkdir)?;
        }
    }

    let target = linkdir.join(target_filename(file));

    if target.exists() && !opts.noop {
        fs::remove_file(&target)?;
    }

    println!("Symlink: {} -> {}", file.file_name().unwrap(), target);

    if !opts.noop {
        symlink(file, &target)?;
    }
    Ok(true)
}

fn jpgs_in(dir: &Utf8Path) -> anyhow::Result<Vec<Utf8PathBuf>> {
    let files = dir
        .read_dir_utf8()?
        .filter_map(Result::ok)
        .map(|e| e.path().to_path_buf())
        .filter(|p| {
            p.is_file()
                && matches!(p.extension(), Some(ext) if ext.eq_ignore_ascii_case("jpg") 
                || ext.eq_ignore_ascii_case("jpeg"))
        })
        .collect();

    Ok(files)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::{defopts, fixture};
    use camino_tempfile_ext::prelude::*;

    #[test]
    fn test_resize_no_action() {
        assert!(
            !resize_or_link(
                &fixture("commands/artfix/tester.good_art/front.jpg"),
                &Utf8PathBuf::from("/tmp"),
                &defopts()
            )
            .unwrap()
        );
    }

    #[test]
    fn test_resize() {
        let file_name = "front.jpg";
        let linkdir = Utf8PathBuf::from("/tmp");
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/artfix/tester.too_big"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        let before = ImageReader::open(&file_under_test)
            .unwrap()
            .decode()
            .unwrap();
        let (x, y) = before.dimensions();

        assert_eq!(x, 900);
        assert_eq!(y, 900);

        assert!(resize_or_link(&file_under_test, &linkdir, &defopts()).unwrap());

        let after = ImageReader::open(&file_under_test)
            .unwrap()
            .decode()
            .unwrap();

        let (x1, y1) = after.dimensions();

        assert_eq!(x1, MAX_ARTWORK_SIZE);
        assert_eq!(y1, MAX_ARTWORK_SIZE);
    }

    #[test]
    fn test_symlink() {
        let source_file = fixture("commands/artfix/tester.not_square/front.jpg");
        let target_dir = Utf8TempDir::new().unwrap();
        let expected_file = target_dir.path().join(target_filename(&source_file));

        assert!(resize_or_link(&source_file, target_dir.path(), &defopts()).unwrap());
        assert!(expected_file.exists());
        assert!(expected_file.is_symlink());
    }

    #[test]
    fn test_rename() {
        let dir_name = "tester.wrong_name";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/artfix"), &["tester.wrong_name/**/*"])
            .unwrap();

        let dir_under_test = tmp.path().join(dir_name);
        let expected_artwork = tmp.path().join("front.jpg");

        assert!(dir_under_test.join("cover.jpg").exists());
        assert!(!expected_artwork.exists());
        assert!(rename(&dir_under_test, &expected_artwork, &defopts()).unwrap());
        assert!(!dir_under_test.join("cover.jpg").exists());
        assert!(expected_artwork.exists());
        assert!(!rename(&dir_under_test, &expected_artwork, &defopts()).unwrap());
    }
}
