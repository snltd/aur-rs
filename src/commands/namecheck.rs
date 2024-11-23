use crate::utils::dir;
use crate::utils::metadata::AurMetadata;
use crate::utils::string::Compacted;
use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub fn run(root_dir: &str) -> anyhow::Result<()> {
    for name_cluster in check_names(root_dir.to_string())? {
        println!("{}", format_dupes(&name_cluster));
    }

    Ok(())
}

fn check_names(root_dir: String) -> anyhow::Result<Vec<Vec<String>>> {
    let all_files = dir::expand_file_list(&[root_dir], true)?;

    if all_files.is_empty() {
        return Err(anyhow!("No files found"));
    }

    let unique_artists = artist_list(all_files)?;

    let mut ret = check_thes(&unique_artists);
    ret.extend(check_compacted(&unique_artists));

    Ok(ret)
}

fn artist_list(file_hash: HashSet<PathBuf>) -> anyhow::Result<HashSet<String>> {
    println!("Getting Artist list");
    let mut unique_artists: HashSet<String> = HashSet::new();

    for file in file_hash {
        let info = AurMetadata::new(&file)?;
        unique_artists.insert(info.tags.artist);
    }
    println!("finished Getting Artist list");

    Ok(unique_artists)
}

fn check_thes(artists: &HashSet<String>) -> Vec<Vec<String>> {
    println!("checking thes");
    let thes = artists.iter().filter(|a| a.starts_with("The "));

    let mut maybe_the_same: Vec<Vec<String>> = Vec::new();

    for the in thes {
        let un_thed = the.replacen("The ", "", 1);
        if artists.contains(&un_thed) {
            maybe_the_same.push(vec![the.to_owned(), un_thed]);
        }
    }
    println!("finished checking thes");

    maybe_the_same
}

fn check_compacted(artists: &HashSet<String>) -> Vec<Vec<String>> {
    println!("checking compacted");
    let mut seen: HashMap<String, Vec<String>> = HashMap::new();
    // compacted -> [artist_1, artist_2...]

    for artist in artists {
        let compacted = artist.compacted();
        seen.entry(compacted).or_default().push(artist.to_owned());
    }

    let ret = seen
        .values()
        .filter(|v| v.len() > 1)
        .map(|v| v.to_vec())
        .collect();

    println!("finished checking compacted");
    ret
}

fn format_dupes(dupe_cluster: &[String]) -> String {
    let mut ret = dupe_cluster.first().unwrap().to_string();
    dupe_cluster[1..]
        .iter()
        .for_each(|d| ret.push_str(&format!("\n  {}", d)));
    ret.push('\n');

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_unordered::assert_eq_unordered;
    use aur::test_utils::spec_helper::fixture_as_string;

    #[test]
    fn test_artist_list() {
        let fixture_dir_flac = fixture_as_string("commands/namecheck/flac");
        let all_files = dir::expand_file_list(&[fixture_dir_flac], true).unwrap();

        let mut expected_flac: HashSet<String> = HashSet::new();
        expected_flac.insert("Singer".to_string());
        expected_flac.insert("Artist".to_string());
        expected_flac.insert("The Artist".to_string());

        assert_eq!(expected_flac, artist_list(all_files).unwrap());

        let fixture_dir_mp3 = fixture_as_string("commands/namecheck/mp3");
        let all_files = dir::expand_file_list(&[fixture_dir_mp3], true).unwrap();

        let mut expected_mp3: HashSet<String> = HashSet::new();
        expected_mp3.insert("The B52's".to_string());
        expected_mp3.insert("The B-52's".to_string());
        expected_mp3.insert("The B52s".to_string());
        assert_eq!(expected_mp3, artist_list(all_files).unwrap());
    }

    #[test]
    fn test_check_thes_matches() {
        let mut input: HashSet<String> = HashSet::new();
        input.insert("Singer".to_string());
        input.insert("Artist".to_string());
        input.insert("The Artist".to_string());
        input.insert("Merpers".to_string());
        input.insert("The Merp".to_string());
        input.insert("The Null Set".to_string());
        input.insert("Null Set".to_string());

        let mut expected = [
            vec!["The Artist".to_string(), "Artist".to_string()],
            vec!["The Null Set".to_string(), "Null Set".to_string()],
        ];

        let mut result = check_thes(&input);
        expected.sort();
        result.sort();

        assert_eq_unordered!(&expected[0], &result[0]);
        assert_eq_unordered!(&expected[1], &result[1]);
    }

    #[test]
    fn test_check_no_matches() {
        let mut input: HashSet<String> = HashSet::new();
        input.insert("Singer".to_string());
        input.insert("Artist".to_string());
        input.insert("The Merp".to_string());
        input.insert("The Null Set".to_string());

        let expected: Vec<Vec<String>> = Vec::new();

        assert_eq!(expected, check_thes(&input));
    }

    #[test]
    fn test_check_compacted() {
        let mut input: HashSet<String> = HashSet::new();
        input.insert("The B52's".to_string());
        input.insert("The B-52's".to_string());
        input.insert("B-52's".to_string());
        input.insert("The Merpers".to_string());
        input.insert("The B52s".to_string());

        let expected = [vec![
            "The B52's".to_string(),
            "The B-52's".to_string(),
            "The B52s".to_string(),
        ]];

        assert_eq_unordered!(&expected[0], &check_compacted(&input)[0]);
    }
}
