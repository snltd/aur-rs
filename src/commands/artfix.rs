use crate::utils::config::{ARTWORK_QUALITY, MAX_ARTWORK_SIZE};
use crate::utils::dir;
use crate::utils::helpers::MaybeProgress;
use crate::utils::types::GlobalOpts;
use crate::{err_if_empty, verbose};
use anyhow::{Context, anyhow};
use camino::{Utf8Path, Utf8PathBuf};
use imagesize;
use indicatif::ProgressBar;
use jpeg_decoder::Decoder;
use jpeg_encoder::{ColorType, Encoder};
use resize::{Pixel::RGB8, Type::Lanczos3};
use rgb::{ComponentBytes, FromSlice, RGB};
use std::fs;
use std::os::unix::fs::symlink;

pub fn run(
    dirlist: &[Utf8PathBuf],
    recurse: bool,
    linkdir: Utf8PathBuf,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let dirs = dir::expand_dir_list(dirlist, recurse);
    err_if_empty!(dirs);

    let mut ret_code = true;

    let pb = if recurse {
        MaybeProgress::Bar(ProgressBar::new(dirs.len() as u64))
    } else {
        MaybeProgress::Direct
    };

    for dir in dirs {
        pb.inc(1);
        if check_artwork(&dir, &linkdir, &pb, opts).is_err() {
            ret_code = false;
        }
    }

    pb.finish();
    Ok(ret_code)
}

fn check_artwork(
    dir: &Utf8Path,
    linkdir: &Utf8Path,
    pb: &MaybeProgress,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let expected_artwork = dir.join("front.jpg");

    let mut changes: Vec<bool> = Vec::new();

    if !expected_artwork.exists() {
        changes.push(rename(dir, &expected_artwork, pb, opts)?);
    }

    if expected_artwork.exists() {
        changes.push(resize_or_link(&expected_artwork, linkdir, pb, opts)?);
    }

    if changes.iter().any(|c| *c) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn rename(
    dir: &Utf8Path,
    front: &Utf8Path,
    pb: &MaybeProgress,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let mut ret = false;

    // Yes, this will flatten multiple files into one. That's fine.
    for file in jpgs_in(dir)? {
        if file != front {
            if !opts.quiet {
                pb.println(&format!("Rename: {} -> front.jpg", file));
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
fn resize_or_link(
    file: &Utf8Path,
    linkdir: &Utf8Path,
    pb: &MaybeProgress,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
    let img_size = imagesize::size(file)?;

    if img_size.width != img_size.height {
        return symlink_art(file, linkdir, pb, opts);
    }

    if img_size.width <= MAX_ARTWORK_SIZE {
        return Ok(false);
    }

    if !opts.quiet {
        pb.println(&format!(
            "Resize: {} -> {s}x{s}",
            file,
            s = MAX_ARTWORK_SIZE
        ));
    }

    if !opts.noop {
        resize_art(file, img_size.width)?;
        // let new_img = img.resize_exact(MAX_ARTWORK_SIZE, MAX_ARTWORK_SIZE, Lanczos3);
        // new_img.save(file)?;
    }

    Ok(true)
}

// Resizes a square image
fn resize_art(path: &Utf8Path, orig_size: usize) -> anyhow::Result<()> {
    let bytes = fs::read(path)?;
    let mut decoder = Decoder::new(bytes.as_slice());
    let pixels = decoder.decode()?;
    let info = decoder.info().context("failed to get JPEG info")?;

    let color_type = match info.pixel_format {
        jpeg_decoder::PixelFormat::RGB24 => ColorType::Rgb,
        jpeg_decoder::PixelFormat::L8 => ColorType::Luma,
        fmt => return Err(anyhow!("unsupported pixel format: {fmt:?}")),
    };

    let mut output = Vec::new();
    let encoder = Encoder::new(&mut output, ARTWORK_QUALITY);

    match color_type {
        ColorType::Rgb => {
            let src: &[RGB<u8>] = pixels.as_rgb();
            let mut dst = vec![RGB::new(0u8, 0, 0); MAX_ARTWORK_SIZE * MAX_ARTWORK_SIZE];
            resize::new(
                orig_size,
                orig_size,
                MAX_ARTWORK_SIZE,
                MAX_ARTWORK_SIZE,
                RGB8,
                Lanczos3,
            )?
            .resize(src, &mut dst)?;
            encoder.encode(
                dst.as_bytes(),
                MAX_ARTWORK_SIZE as u16,
                MAX_ARTWORK_SIZE as u16,
                ColorType::Rgb,
            )?;
        }
        ColorType::Luma => {
            use resize::Pixel::Gray8;
            use rgb::alt::Gray;
            let src: &[Gray<u8>] = bytemuck::cast_slice(&pixels);
            let mut dst = vec![Gray(0u8); MAX_ARTWORK_SIZE * MAX_ARTWORK_SIZE];
            resize::new(
                orig_size,
                orig_size,
                MAX_ARTWORK_SIZE,
                MAX_ARTWORK_SIZE,
                Gray8,
                Lanczos3,
            )?
            .resize(src, &mut dst)?;
            encoder.encode(
                bytemuck::cast_slice(&dst),
                MAX_ARTWORK_SIZE as u16,
                MAX_ARTWORK_SIZE as u16,
                ColorType::Luma,
            )?;
        }
        _ => unreachable!(),
    }

    fs::write(path, &output)?;
    Ok(())
}

fn target_filename(file: &Utf8Path) -> String {
    file.to_string()
        .replace('/', "-")
        .trim_start_matches('-')
        .into()
}

fn symlink_art(
    file: &Utf8Path,
    linkdir: &Utf8Path,
    pb: &MaybeProgress,
    opts: &GlobalOpts,
) -> anyhow::Result<bool> {
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

    pb.println(&format!(
        "Symlink: {} -> {}",
        file.file_name().unwrap(),
        target
    ));

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
    use camino_tempfile_ext::prelude::*;
    use snltest::fixture;

    #[test]
    fn test_resize_no_action() {
        assert!(
            !resize_or_link(
                &fixture!("commands/artfix/tester.good_art/front.jpg"),
                &Utf8PathBuf::from("/tmp"),
                &MaybeProgress::Direct,
                &GlobalOpts::default()
            )
            .unwrap()
        );
    }

    #[test]
    fn test_resize() {
        let file_name = "front.jpg";
        let linkdir = Utf8PathBuf::from("/tmp");
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/artfix/tester.too_big"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let before = imagesize::size(&file_under_test).unwrap();

        assert_eq!(before.width, 900);
        assert_eq!(before.height, 900);

        assert!(
            resize_or_link(
                &file_under_test,
                &linkdir,
                &MaybeProgress::Direct,
                &GlobalOpts::default()
            )
            .unwrap()
        );

        let after = imagesize::size(&file_under_test).unwrap();

        assert_eq!(after.width, MAX_ARTWORK_SIZE);
        assert_eq!(after.height, MAX_ARTWORK_SIZE);
    }

    #[test]
    fn test_symlink() {
        let source_file = fixture!("commands/artfix/tester.not_square/front.jpg");
        let target_dir = Utf8TempDir::new().unwrap();
        let expected_file = target_dir.path().join(target_filename(&source_file));

        assert!(
            resize_or_link(
                &source_file,
                target_dir.path(),
                &MaybeProgress::Direct,
                &GlobalOpts::default()
            )
            .unwrap()
        );
        assert!(expected_file.exists());
        assert!(expected_file.is_symlink());
    }

    #[test]
    fn test_rename() {
        let dir_name = "tester.wrong_name";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/artfix"), &["tester.wrong_name/**/*"])
            .unwrap();

        let dir_under_test = tmp.path().join(dir_name);
        let expected_artwork = tmp.path().join("front.jpg");

        assert!(dir_under_test.join("cover.jpg").exists());
        assert!(!expected_artwork.exists());
        assert!(
            rename(
                &dir_under_test,
                &expected_artwork,
                &MaybeProgress::Direct,
                &GlobalOpts::default()
            )
            .unwrap()
        );
        assert!(!dir_under_test.join("cover.jpg").exists());
        assert!(expected_artwork.exists());
        assert!(
            !rename(
                &dir_under_test,
                &expected_artwork,
                &MaybeProgress::Direct,
                &GlobalOpts::default()
            )
            .unwrap()
        );
    }
}
