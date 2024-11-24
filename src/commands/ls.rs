use crate::utils::dir::expand_dir_list;
use crate::utils::metadata::{AurMetadata, AurTags};
use crate::utils::term::term_width;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(dirlist: &[String], recurse: bool) -> anyhow::Result<()> {
    // If no argument is given, default to the cwd, just like real ls(1).
    let dirs = if dirlist.is_empty() {
        &[std::env::current_dir()?.to_string_lossy().to_string()]
    } else {
        dirlist
    };

    let dirs_to_list: Vec<PathBuf> = dirs.to_vec().iter().map(|d| PathBuf::from(d)).collect();
    for dir in expand_dir_list(&dirs_to_list, recurse) {
        print_listing(list_info(&dir)?);
    }
    Ok(())
}

fn list_info(dir: &Path) -> anyhow::Result<Vec<String>> {
    let entries = fs::read_dir(dir)?;

    let mut all_file_tags: Vec<AurTags> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let file = entry.path();
            AurMetadata::new(&file).ok().map(|metadata| metadata.tags)
        })
        .collect();

    all_file_tags.sort_by_key(|tags| tags.t_num);
    let width = term_width();
    let ret = all_file_tags
        .iter()
        .map(|t| format_line(t, width))
        .collect();
    Ok(ret)
}

fn format_line(tags: &AurTags, width: usize) -> String {
    let title_width = width / 2;
    let artist_width = width / 4;
    let album_col = width - title_width - artist_width - 5;

    format!(
        "{:02} {:artist_width$} {:title_width$} {:>album_col$}",
        tags.t_num, tags.artist, tags.title, tags.album
    )
}

fn print_listing(lines: Vec<String>) {
    println!("{}", lines.join("\n"));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;

    #[test]
    fn test_list_info() {
        let result = list_info(&fixture("commands/ls")).unwrap();
        assert_eq!(3, result.len());
        assert!(result[0].starts_with("01 List Artist"));
        assert!(result[1].starts_with("02 List Artist"));
        assert!(result[2].starts_with("03 List Artist"));
        assert!(result[0].ends_with("List Album"));
    }

    #[test]
    fn test_format_line() {
        let tags = AurTags {
            artist: "Artist".to_string(),
            title: "Test Title".to_string(),
            album: "Test Album".to_string(),
            t_num: 4,
            year: 2024,
            genre: "Test".to_string(),
        };

        assert_eq!(
            "04 Artist  Test Title      Test Album".to_string(),
            format_line(&tags, 30)
        );

        assert_eq!(
            "04 Artist          Test Title                     Test Album".to_string(),
            format_line(&tags, 60)
        );
    }
}
