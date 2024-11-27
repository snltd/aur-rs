use crate::utils::string::Capitalize;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::fs::read_to_string;

pub static WORDS: Lazy<Words> = Lazy::new(|| init_words());

type InBrackets = bool;
struct TagMaker();

impl TagMaker {
    pub fn title_from(string: &str) -> String {
        let mut in_brackets = false;
        let words: Vec<_> = string.split(['_', ' ']).collect();
        let mut bits: Vec<String> = Vec::new();
        let word_count = words.len();

        for (i, word) in words.iter().enumerate() {
            let (new_word, ib) = handle_string(word, i, word_count, in_brackets);
            bits.push(new_word);
            in_brackets = ib;
        }

        join_up(&bits, in_brackets)
    }

    pub fn artist_from(string: &str) -> String {
        Self::title_from(string).replace("and the ", "and The ")
    }

    pub fn album_from(string: &str) -> String {
        Self::title_from(string)
    }

    pub fn genre_from(string: &str) -> String {
        Self::title_from(string).trim().to_string()
    }

    pub fn t_num_from(number: &str) -> u32 {
        number.to_string().parse::<u32>().unwrap_or(0)
    }
}

fn handle_string(
    string: &str,
    index: usize,
    count: usize,
    in_brackets: InBrackets,
) -> (String, InBrackets) {
    let chars: Vec<_> = string.chars().collect();

    if chars.len() >= 3 && chars[0].is_alphabetic() && chars[1] == '-' && chars[2].is_alphabetic() {
        (handle_initials(string), in_brackets)
    } else if string.contains("--") {
        handle_long_dash(string, in_brackets)
    } else if string.contains("-") {
        (handle_short_dash(string, index, count), in_brackets)
    } else {
        (
            smart_capitalize(expand(string).as_str(), index, count),
            in_brackets,
        )
    }
}

fn handle_initials(string: &str) -> String {
    let mut ret = string.to_uppercase().replace("-", ".");
    ret.push('.');
    ret
}

fn expand(word: &str) -> String {
    WORDS
        .expand
        .get(word)
        .map(|s| s.as_str())
        .unwrap_or(word)
        .to_string()
}

fn handle_long_dash(string: &str, in_brackets: InBrackets) -> (String, InBrackets) {
    let words: Vec<_> = string.split("--").collect();

    if in_brackets {
        close_brackets(words)
    } else {
        open_brackets(words, in_brackets)
    }
}

fn handle_short_dash(string: &str, index: usize, count: usize) -> String {
    let words: Vec<_> = string.split("-").collect();
    words
        .iter()
        .map(|w| smart_capitalize(w, index, count))
        .collect::<Vec<String>>()
        .join("-")
}

fn open_brackets(words: Vec<&str>, in_brackets: InBrackets) -> (String, InBrackets) {
    let first_word = expand(words[0]).capitalize();
    let (mut inner_words, in_brackets) = handle_string(words[1], 0, 0, in_brackets);

    if words.len() <= 2 {
        return (format!("{} ({}", first_word, inner_words), true);
    }

    let final_word = expand(words.last().unwrap()).capitalize();

    if words.len() > 3 {
        inner_words = handle_initials(words[1..words.len() - 2].join("-").as_str());
    }

    (
        format!("{} ({}) {}", first_word, inner_words, final_word),
        in_brackets,
    )
}

fn close_brackets(words: Vec<&str>) -> (String, InBrackets) {
    (
        format!(
            "{}) {}",
            expand(words[0]).capitalize(),
            expand(words[1]).capitalize()
        ),
        false,
    )
}

fn smart_capitalize(word: &str, index: usize, count: usize) -> String {
    let chars: Vec<_> = word.chars().collect();
    let lowercase_word = word.to_lowercase();

    if chars.len() > 2 && chars[0].is_alphabetic() && chars[1] == '.' {
        word.to_string()
    } else if WORDS.no_caps.contains(&lowercase_word) && index >= 1 && index <= count - 2 {
        lowercase_word.to_string()
    } else if WORDS.all_caps.contains(&lowercase_word) {
        word.to_uppercase().to_string()
    } else if chars.iter().all(|c| c.is_uppercase() || c.is_numeric()) {
        word.to_string()
    } else {
        word.capitalize()
    }
}

fn join_up(words: &Vec<String>, in_brackets: InBrackets) -> String {
    let mut ret = words.join(" ");

    if in_brackets {
        ret.push(')');
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_title_plain() {
        assert_eq!("Blue Bell Knoll", TagMaker::title_from("blue_bell_knoll"));
        assert_eq!(
            "The Boy with the Arab Strap",
            TagMaker::title_from("the_boy_with_the_arab_strap")
        );
    }

    #[test]
    fn test_title_force_caps() {
        assert_eq!("Enemies EP", TagMaker::title_from("enemies_ep"));
    }

    #[test]
    fn test_title_downcase_at_end() {
        assert_eq!(
            "I Want to Wake Up",
            TagMaker::title_from("i_want_to_wake_up")
        );
    }

    #[test]
    fn test_title_contraction() {
        assert_eq!("Don't Go", TagMaker::title_from("dont_go"));
        assert_eq!(
            "You Can't Hold What You Haven't Got in Your Hand",
            TagMaker::title_from("you_cant_hold_what_you_havent_got_in_your_hand")
        );
    }

    fn test_title_inches() {
        assert_eq!(
            "I Feel Love (12\" Mix)",
            TagMaker::title_from("i_feel_love--12inch_mix")
        );
        assert_eq!(
            "Fugitive (7\" Mix)",
            TagMaker::title_from("fugitive--7inch_mix")
        );
    }

    fn test_title_hyphen() {
        assert_eq!("When-Never", TagMaker::title_from("when-never"));
        assert_eq!(
            "Who-What and Maybe-Not",
            TagMaker::title_from("who-what_and_maybe-not")
        );
        assert_eq!(
            "Tick-Tock-Tick-Tock",
            TagMaker::title_from("tick-tock-tick-tock")
        );
    }

    fn test_title_brackets_in_middle() {
        assert_eq!(
            "This Is (Almost) Too Easy",
            TagMaker::title_from("this_is--almost--too_easy")
        );
        assert_eq!(
            "Can We Make It (Just a Little Bit) Harder",
            TagMaker::title_from("can_we_make_it--just_a_little_bit--harder")
        );
        assert_eq!(
            "Title }ing In (Bracketed Words)",
            TagMaker::title_from("title_}ing_in--bracketed_words")
        );
        assert_eq!(
            "Two (Lots) Of Brackets (Is Tricky)",
            TagMaker::title_from("two--lots--of_brackets--is_tricky")
        );
        assert_eq!(
            "Variations 3 (Canon on the Unison)",
            TagMaker::title_from("variations_3--canon_on_the_unison")
        );
    }

    fn test_title_brackets_at_end() {
        assert_eq!(
            "Suburbia (The Full Horror)",
            TagMaker::title_from("suburbia--the_full_horror")
        );

        assert_eq!(
            "Om Mani Padme Hum 3 (Piano Version)",
            TagMaker::title_from("om_mani_padme_hum_3--piano_version")
        );
        assert_eq!(
            "Drumming (Part III)",
            TagMaker::title_from("drumming--part_iii")
        );
    }

    fn test_title_initials() {
        assert_eq!("C.R.E.E.P.", TagMaker::title_from("c-r-e-e-p"));
        assert_eq!("The N.W.R.A.", TagMaker::title_from("the_n-w-r-a"));
        assert_eq!("W.M.C. Blob 59", TagMaker::title_from("w-m-c_blob_59"));
    }

    fn test_initials_in_brackets() {
        assert_eq!(
            "The (I.N.I.T.I.A.L.S.) In Brackets",
            TagMaker::title_from("the--i-n-i-t-i-a-l-s--in_brackets")
        );
    }

    #[test]
    fn test_artist() {
        assert_eq!("Stereolab", TagMaker::artist_from("stereolab"));
        assert_eq!("Royal Trux", TagMaker::artist_from("royal_trux"));
        assert_eq!(
            "Jeffrey Lewis and The Bolts",
            TagMaker::artist_from("jeffrey_lewis_and_the_bolts")
        );
        assert_eq!(
            "Someone featuring Someone Else",
            TagMaker::artist_from("someone_ft_someone_else")
        );
        assert_eq!("R.E.M.", TagMaker::artist_from("r-e-m"));
        assert_eq!("M.J. Hibbett", TagMaker::artist_from("m-j_hibbett"));
        // assert_eq!("ABBA", TagMaker::artist_from("abba"));
        // assert_eq!("Add N to (X)", TagMaker::artist_from("add_n_to_"));
    }

    #[test]
    fn test_album() {
        assert_eq!("Spiderland", TagMaker::album_from("spiderland"));
        assert_eq!(
            "Mars Audiac Quintet",
            TagMaker::album_from("mars_audiac_quintet")
        );
        assert_eq!(
            "The Decline and Fall of Heavenly",
            TagMaker::album_from("the_decline_and_fall_of_heavenly")
        );
    }

    #[test]
    fn test_genre() {
        assert_eq!("Noise", TagMaker::genre_from("noise"));
        assert_eq!("Noise", TagMaker::genre_from("Noise "));
        assert_eq!("Hip-Hop", TagMaker::genre_from("Hip-Hop"));
        assert_eq!("Rock and Roll", TagMaker::genre_from("rock AND roll"));
    }

    #[test]
    fn test_t_num() {
        assert_eq!(1, TagMaker::t_num_from("01"));
        assert_eq!(39, TagMaker::t_num_from("39"));
    }
}

#[derive(Deserialize, Debug)]
pub struct Words {
    pub no_caps: HashSet<String>,
    pub all_caps: HashSet<String>,
    pub ignore_case: HashSet<String>,
    pub expand: HashMap<String, String>,
}

fn init_words() -> Words {
    let words_file = current_dir()
        .unwrap()
        .join("src")
        .join("static")
        .join("words.toml");
    let raw = read_to_string(&words_file)
        .unwrap_or_else(|_| panic!("could not read words file {}", words_file.display()));
    toml::from_str(&raw)
        .map_err(|e| anyhow::anyhow!(e))
        .expect("could not parse words file")
}
