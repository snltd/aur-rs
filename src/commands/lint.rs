use crate::utils::config::load_config;
use crate::utils::dir::{expand_file_list, media_files};
use crate::utils::metadata::{AurMetadata, AurTags, RawTags};
use crate::utils::tag_validator::TagValidator;
use crate::utils::types::GlobalOpts;
use crate::utils::words::Words;
use std::collections::HashSet;
use std::path::Path;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
enum CheckResult {
    Good,
    Bad(String),
}

pub fn run(files: &[String], recurse: bool, opts: &GlobalOpts) -> anyhow::Result<()> {
    let config = load_config(&opts.config)?;
    let words = Words::new(&config);
    let validator = TagValidator::new(&words);
    for f in media_files(expand_file_list(files, recurse)?) {
        process_results(lint_file(&f, &validator)?);
    }
    Ok(())
}

fn process_results(results: Vec<CheckResult>) {
    let failures: Vec<_> = results
        .iter()
        .filter_map(|r| match r {
            CheckResult::Bad(reason) => Some(reason),
            _ => None,
        })
        .collect();

    if failures.is_empty() {
        println!("All checks passed!");
    } else {
        println!("Checks failed for the following reasons:");
        for failure in failures {
            println!("- {}", failure);
        }
    }
}

fn lint_file(file: &Path, validator: &TagValidator) -> anyhow::Result<Vec<CheckResult>> {
    let info = AurMetadata::new(file)?;
    let results: Vec<_> = run_checks(&info, validator)
        .into_iter()
        .filter(|r| matches!(r, CheckResult::Bad(_)))
        .collect();
    Ok(results)
}

fn run_checks(metadata: &AurMetadata, validator: &TagValidator) -> Vec<CheckResult> {
    vec![
        has_valid_name(&metadata.filename),
        has_no_unwanted_tags(&metadata.filetype, &metadata.rawtags),
        has_no_picture(metadata.has_picture),
    ]
    .into_iter()
    .chain(has_no_invalid_tags(&metadata.tags, validator))
    .collect()
}

fn has_no_picture(has_picture: bool) -> CheckResult {
    if has_picture {
        CheckResult::Bad("has embedded artwork".into())
    } else {
        CheckResult::Good
    }
}

fn has_valid_name(fname: &str) -> CheckResult {
    let chunks: Vec<_> = fname.split('.').collect();

    if chunks.len() == 4
        && chunks.iter().all(|c| is_safe(c))
        && !chunks[1].starts_with("the_")
        && is_safe_num(chunks[0])
    {
        CheckResult::Good
    } else {
        CheckResult::Bad("invalid name".into())
    }
}

fn has_no_unwanted_tags(filetype: &str, rawtags: &RawTags) -> CheckResult {
    let tag_keys: HashSet<String> = rawtags.iter().map(|(k, _v)| k).cloned().collect();

    let expected_tags = match filetype {
        "flac" => HashSet::from([
            "artist".into(),
            "album".into(),
            "title".into(),
            "tracknumber".into(),
            "genre".into(),
            "date".into(),
        ]),
        "mp3" => HashSet::from([
            "tpe1".into(),
            "talb".into(),
            "tit2".into(),
            "trck".into(),
            "tyer".into(),
            "tcon".into(),
        ]),
        _ => return CheckResult::Bad("unrecognised filetype".into()),
    };

    let irrelevant_tags = HashSet::from(["encoder".into(), "blank".into()]);
    let allowed_tags: HashSet<_> = expected_tags.union(&irrelevant_tags).cloned().collect();
    let unexpected_tags: HashSet<_> = tag_keys.difference(&allowed_tags).collect();

    if unexpected_tags.is_empty() {
        CheckResult::Good
    } else {
        let mut tag_vec: Vec<String> = unexpected_tags.iter().map(|s| s.to_string()).collect();
        tag_vec.sort();
        CheckResult::Bad(format!("unexpected tags: {}", tag_vec.join(", ")))
    }
}

fn has_no_invalid_tags(tags: &AurTags, validator: &TagValidator) -> Vec<CheckResult> {
    let mut problems: Vec<CheckResult> = Vec::new();

    if !validator.validate_title(&tags.title) {
        problems.push(CheckResult::Bad(format!("invalid title: {}", tags.title)));
    }

    if !validator.validate_artist(&tags.artist) {
        problems.push(CheckResult::Bad(format!("invalid artist: {}", tags.artist)));
    }

    if !validator.validate_album(&tags.album) {
        problems.push(CheckResult::Bad(format!("invalid album: {}", tags.album)));
    }

    if !validator.validate_t_num(&tags.t_num.to_string()) {
        problems.push(CheckResult::Bad(format!("invalid t_num: {}", tags.t_num)));
    }

    if !validator.validate_year(&tags.year.to_string()) {
        problems.push(CheckResult::Bad(format!("invalid year: {}", tags.year)));
    }

    if !validator.validate_genre(&tags.genre) {
        problems.push(CheckResult::Bad(format!("invalid genre: {}", tags.genre)));
    }

    if problems.is_empty() {
        vec![CheckResult::Good]
    } else {
        problems
    }
}

fn is_safe(chunk: &str) -> bool {
    if chunk.starts_with(['-', '_']) || chunk.ends_with(['-', '_']) || chunk.contains("__") {
        return false;
    }

    chunk
        .chars()
        .all(|c| c == '_' || c == '-' || (c.is_alphabetic() && c.is_lowercase()) || c.is_numeric())
}

fn is_safe_num(chunk: &str) -> bool {
    chunk.len() == 2 && chunk != "00" && chunk.chars().all(|c| c.is_numeric())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::{fixture, sample_config};

    #[test]
    fn test_has_valid_name() {
        assert!(matches!(
            has_valid_name("01.artist.title.flac"),
            CheckResult::Good,
        ));
        assert!(matches!(
            has_valid_name("01.artist.title.mp3"),
            CheckResult::Good
        ));
        assert!(matches!(
            has_valid_name("19.my_favourite_band.their_best_song.flac"),
            CheckResult::Good
        ));
        assert!(matches!(
            has_valid_name("03.123.456.flac"),
            CheckResult::Good
        ));
        assert!(matches!(
            has_valid_name("05.a_band.a_song-with_brackets.flac"),
            CheckResult::Good
        ));
        assert!(matches!(
            has_valid_name("07.some_singer.i-n-i-t-i-a-l-s.flac"),
            CheckResult::Good
        ));

        assert!(matches!(
            has_valid_name("artist.title.flac"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("01.title.mp3"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("19.my_favourite_band.their_best_song!.flac"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("00.a_band.a_song-with_brackets.flac"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("02.Artist.Title.flac"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("03.someone_&_the_somethings.song.mp3"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("04.too__many.underscores.flac"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("1.artist.title.flac"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("03._artist.title.mp3"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("03.artist.title_.mp3"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("03.artist.title_(with_brackets).flac"),
            CheckResult::Bad(_)
        ));
        assert!(matches!(
            has_valid_name("07.the_somethings.i-n-i-t-i-a-l-s.flac"),
            CheckResult::Bad(_)
        ));
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe("artist"));
        assert!(is_safe("01"));
        assert!(is_safe("a"));
        assert!(is_safe("7"));
        assert!(is_safe("me"));
        assert!(is_safe("two_words"));
        assert!(is_safe("and_three_words"));
        assert!(is_safe("some--bracketed--words"));
        assert!(is_safe("with-hyphen"));
        assert!(is_safe("1_two_3"));
        assert!(is_safe("one_2_3"));

        assert!(!is_safe("_word"));
        assert!(!is_safe("-word"));
        assert!(!is_safe("_"));
        assert!(!is_safe("-"));
        assert!(!is_safe("word_"));
        assert!(!is_safe("two__words"));
        assert!(!is_safe("tres,comma"));
        assert!(!is_safe("Word"));
    }

    #[test]
    fn test_is_safe_num() {
        assert!(is_safe_num("01"));
        assert!(is_safe_num("99"));
        assert!(!is_safe_num("00"));
    }

    #[test]
    fn lint_functional_tests() {
        let words = Words::new(&sample_config());
        let validator = TagValidator::new(&words);

        assert!(lint_file(
            &fixture("commands/lint/01.tester.lints_fine.flac"),
            &validator
        )
        .unwrap()
        .is_empty());

        assert!(lint_file(
            &fixture("commands/lint/02.tester.lints_fine.mp3"),
            &validator
        )
        .unwrap()
        .is_empty());

        assert_eq!(
            vec![
                CheckResult::Bad("invalid name".into()),
                CheckResult::Bad("invalid title: Missing Genre, Track No, Year".into()),
                CheckResult::Bad("invalid t_num: 0".into()),
                CheckResult::Bad("invalid year: 0".into()),
                CheckResult::Bad("invalid genre: ".into())
            ],
            lint_file(
                &fixture("commands/lint/00.tester.missing_genre_track_no_year.flac"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![CheckResult::Bad(
                "invalid title: File,with Bad Title".into()
            ),],
            lint_file(
                &fixture("commands/lint/04.tester.file_with_bad_title.mp3"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![CheckResult::Bad(
                "unexpected tags: tdrc, tlen, tsse, txxx".into()
            ),],
            lint_file(
                &fixture("commands/lint/05.tester.surplus_tags.mp3"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![
                CheckResult::Bad("unexpected tags: apic, tcom, tenc, tlen, tsse, txxx".into()),
                CheckResult::Bad("has embedded artwork".into()),
            ],
            lint_file(
                &fixture("commands/lint/06.tester.extra_tags_and_picture.mp3"),
                &validator
            )
            .unwrap()
        );

        assert_eq!(
            vec![CheckResult::Bad("has embedded artwork".into())],
            lint_file(&fixture("commands/lint/07.tester.picture.flac"), &validator).unwrap()
        );
        //     let expected_incorrect_tags = vec![
        //         CheckResult::Bad("invalid title: 100hz;Wrong Title".into()),
        //         CheckResult::Bad("invalid artist: Test Tones;Test Artist".into()),
        //     ];

        //     assert_eq!(
        //         expected_incorrect_tags,
        //         lint_file(
        //             &fixture("commands/lint/02.singer_and_the_band.file_for_test.flac"),
        //             &validator
        //         )
        //         .unwrap()
        //     );
        // }
    }
}
