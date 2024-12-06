use crate::utils::dir::expand_dir_list;
use image::imageops::FilterType::Lanczos3;
use image::GenericImageView;
use image::ImageReader;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

const MAX_ARTWORK_SIZE: u32 = 750;

pub fn run(dirlist: &[String], recurse: bool, linkdir: String) -> anyhow::Result<()> {
    let linkdir = PathBuf::from(linkdir);
    let dirlist: Vec<PathBuf> = dirlist.iter().map(PathBuf::from).collect();

    for dir in expand_dir_list(&dirlist, recurse) {
        check_artwork(&dir, &linkdir)?;
    }
    Ok(())
}

fn check_artwork(dir: &Path, linkdir: &Path) -> anyhow::Result<bool> {
    let expected_artwork = dir.join("front.jpg");

    let mut changes: Vec<bool> = Vec::new();

    if !expected_artwork.exists() {
        changes.push(rename(dir, &expected_artwork)?);
    }

    if expected_artwork.exists() {
        changes.push(resize_or_link(&expected_artwork, linkdir)?);
    }

    if changes.iter().any(|c| *c) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn rename(dir: &Path, front: &Path) -> anyhow::Result<bool> {
    let mut ret = false;

    // Yes, this will flatten multiple files into one. That's fine.
    for file in jpgs_in(dir)? {
        if file != front {
            println!("moving {} to {}", file.display(), front.display());
            fs::rename(file, front)?;
            ret = true;
        }
    }

    Ok(ret)
}

// It's not our job to flag up problems, only to fix what we can. Lintdir will point out what
// we can't fix.
fn resize_or_link(file: &Path, linkdir: &Path) -> anyhow::Result<bool> {
    let img = ImageReader::open(file)?.decode()?;
    let (x, y) = img.dimensions();

    if x != y {
        return symlink_art(file, linkdir);
    }

    if x <= MAX_ARTWORK_SIZE {
        return Ok(false);
    }

    println!(
        "Rescaling {} to {s}x{s}",
        file.display(),
        s = MAX_ARTWORK_SIZE
    );
    let new_img = img.resize_exact(MAX_ARTWORK_SIZE, MAX_ARTWORK_SIZE, Lanczos3);
    new_img.save(file)?;
    Ok(true)
}

fn target_filename(file: &Path) -> String {
    file.to_string_lossy()
        .replace('/', "-")
        .trim_start_matches('-')
        .into()
}

fn symlink_art(file: &Path, linkdir: &Path) -> anyhow::Result<bool> {
    if !linkdir.exists() {
        println!("Creating artfix dir: {}", linkdir.display());
        fs::create_dir_all(linkdir)?;
    }

    let target = linkdir.join(target_filename(file));

    if target.exists() {
        fs::remove_file(&target)?;
    }

    println!("Linking {} -> {}", file.display(), target.display());
    symlink(file, &target)?;
    Ok(true)
}

fn jpgs_in(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let files = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|e| e.path())
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
    use crate::utils::spec_helper::fixture;
    use assert_fs::prelude::*;

    #[test]
    fn test_resize_no_action() {
        assert!(!resize_or_link(
            &fixture("commands/artfix/tester.good_art/front.jpg"),
            &PathBuf::from("/tmp")
        )
        .unwrap());
    }

    #[test]
    fn test_resize() {
        let file_name = "front.jpg";
        let linkdir = PathBuf::from("/tmp");
        let tmp = assert_fs::TempDir::new().unwrap();
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

        assert!(resize_or_link(&file_under_test, &linkdir).unwrap());

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
        let target_dir = assert_fs::TempDir::new().unwrap();
        let expected_file = target_dir.join(target_filename(&source_file));
        assert!(resize_or_link(&source_file, &target_dir).unwrap());

        assert!(expected_file.exists());
        assert!(expected_file.is_symlink());
    }

    #[test]
    fn test_rename() {
        let dir_name = "tester.wrong_name";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/artfix"), &["tester.wrong_name/**/*"])
            .unwrap();
        let dir_under_test = tmp.path().join(dir_name);
        let expected_artwork = tmp.path().join("front.jpg");

        assert!(dir_under_test.join("cover.jpg").exists());
        assert!(!expected_artwork.exists());

        assert!(rename(&dir_under_test, &expected_artwork).unwrap());
        assert!(!dir_under_test.join("cover.jpg").exists());
        assert!(expected_artwork.exists());

        assert!(!rename(&dir_under_test, &expected_artwork).unwrap());
    }
}
