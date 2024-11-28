use crate::utils::string::Capitalize;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::fs::read_to_string;
/*

pub static WORDS: Lazy<Words> = Lazy::new(|| init_words());

// pub trait Titlecase {
//     fn titlecase(&self) -> String;
// }

// impl Titlecase for String {
//     fn titlecase(&self,) -> String {
//         // if starts_with()
//         if is_downcase(self,
//         capitalize(self)
//     }
// }

// wrapper
fn titlecase(word: &str) -> String {
    // if starts_with()
    word.capitalize()
}

fn is_ignore_case(word: &str) -> bool {
    WORDS.ignore_case.contains(word)
}

fn is_downcase(word: &str, run_together: bool, previous_word: &str) -> bool {
    WORDS.no_caps.contains(word)
        && (run_together || !previous_word.ends_with([':', '-', '/', ')', '?', '!']))
}

fn is_upcase(word: &str, next_word: &str) -> bool {
    if WORDS.all_caps.contains(word) {
        return true;
    }

    let chars: Vec<_> = next_word.chars().collect();

    if chars.len() >= 4
        && chars[0].is_alphanumeric()
        && chars[1] == '.'
        && chars[2].is_alphanumeric()
        && chars[3] == '.'
    {
        true
    } else {
        chars.len() >= 2 && chars[0].is_alphanumeric() && chars[1] == ','
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_upcase() {
        assert!(is_upcase("uk", "garage"));
        assert!(is_upcase("dvd", "player"));
        assert!(!is_upcase("unit", "test"));
        assert!(is_upcase("countdown", "u.k."));
        assert!(is_upcase("and", "i,"));
    }

    #[test]
    fn test_is_ignore_case() {
        assert!(is_ignore_case("x"));
        assert!(!is_ignore_case("rust"));
    }

    #[test]
    fn test_is_downcase() {
        assert!(is_downcase("the", false, "and"));
        assert!(!is_downcase("cat", false, "the"));
    }

    #[test]
    fn test_titlecase() {
        assert_eq!("Word".to_string(), titlecase("word"));
        assert_eq!("Word".to_string(), titlecase("Word"));
        // assert_eq!("the".to_string(), "The".titlecase());
        // assert_eq!("of".to_string(), "of".titlecase());
        // assert_eq!("and,".to_string(), "And,".titlecase());
        // assert_eq!("(Disc,".to_string(), "(disc,".titlecase());
        // assert_eq!("I".to_string(), "i".titlecase());
        // assert_eq!("a".to_string(), "a".titlecase());
        // assert_eq!("L.A.".to_string(), "l.a.".titlecase());
        // assert_eq!("P.R.O.D.U.C.T.".to_string(), "p.r.o.d.u.c.t.".titlecase());
        // assert_eq!("(B.M.R.".to_string(), "(B.M.R.".titlecase());
        // assert_eq!("(A".to_string(), "(A".titlecase());
        // assert_eq!("(LP".to_string(), "(LP".titlecase());
        // assert_eq!("(Live)".to_string(), "(live)".titlecase());
        // assert_eq!("(II)".to_string(), "(Ii)".titlecase());
        // assert_eq!("OK".to_string(), "ok".titlecase());
        // assert_eq!("Aikea-Guinea".to_string(), "Aikea-Guinea".titlecase());
        // assert_eq!("Itchy+Scratchy".to_string(), "Itchy+scratchy".titlecase());
        // assert_eq!("A-O".to_string(), "A-O".titlecase());
        // assert_eq!("As)".to_string(), "as".titlecase());
        // assert_eq!("A,".to_string(), "A,".titlecase());
        // assert_eq!("Fixed::Content".to_string(), "fixed::content".titlecase());
        // assert_eq!("Kill-a-Man".to_string(), "kill-a-man".titlecase());
        // assert_eq!("Master=Dik".to_string(), "Master=dik".titlecase());
    }
}
// assert_eq!("The", "the".titlecase("as"));
// assert_eq!("A:", "A:".titlecase("men?"));
// assert_eq!("The", "the".titlecase("A:"));
// assert_eq!("The", "the".titlecase("/"));

// impl Titlecase for &str {
//     fn titlecase(&self) -> String {
//         self.to_string().titlecase()
//     }
// }

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
*/
