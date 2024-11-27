use crate::utils::string::Capitalize;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env::current_dir;
use std::fs::read_to_string;

pub static WORDS: Lazy<Words> = Lazy::new(|| init_words());

struct TagMaker {
    in_brackets: bool,
}

impl TagMaker {
    fn new() -> Self {
        Self { in_brackets: false }
    }

    pub fn title_from(&mut self, string: &str) -> String {
        self.in_brackets = false;
        let str: Vec<_> = string.split('_').collect();

        let bits: Vec<String> = str
            .iter()
            .enumerate()
            .map(|(i, s)| self.handle_string(s, i, str.len()))
            .collect();

        self.join_up(&bits)
    }

    pub fn artist_from(&mut self, string: &str) -> String {
        self.title_from(string).replace("and the ", "and The ")
    }

    pub fn album_from(&mut self, string: &str) -> String {
        self.title_from(string)
    }

    pub fn t_num_from(&mut self, number: &str) -> u32 {
        number.to_string().parse::<u32>().unwrap_or(0)
    }

    fn handle_string(&mut self, string: &str, index: usize, count: usize) -> String {
        // println!("handling string {}", string);
        let chars: Vec<_> = string.chars().collect();

        if chars.len() >= 3
            && chars[0].is_alphabetic()
            && chars[1] == '-'
            && chars[2].is_alphabetic()
        {
            self.handle_initials(string)
        } else if string.contains("--") {
            self.handle_long_dash(string)
        } else if string.contains("-") {
            self.handle_short_dash(string, index, count)
        } else {
            self.smart_capitalize(self.expand(string).as_str(), index, count)
        }
    }

    fn expand(&self, string: &str) -> String {
        // println!("expanding {}", string);
        WORDS
            .expand
            .get(string)
            .map(|s| s.as_str())
            .unwrap_or(string)
            .to_string()
    }

    fn handle_initials(&self, string: &str) -> String {
        let mut ret = string.to_uppercase().replace("-", ".");
        ret.push('.');
        ret
    }

    fn handle_long_dash(&mut self, string: &str) -> String {
        let words: Vec<_> = string.split("--").collect();

        if self.in_brackets {
            self.close_brackets(words)
        } else {
            self.open_brackets(words)
        }
    }

    fn close_brackets(&mut self, words: Vec<&str>) -> String {
        println!("CLOSING BRACKETS");
        self.in_brackets = false;
        format!(
            "{}) {}",
            self.expand(words[0]).capitalize(),
            self.expand(words[1]).capitalize()
        )
    }

    fn open_brackets(&mut self, words: Vec<&str>) -> String {
        println!("OPENING BRACKETS");
        self.in_brackets = true;
        let first_word = self.expand(words[0]).capitalize();
        let mut inner_words = self.handle_string(words[1], 0, 0);

        if words.len() <= 2 {
            return format!("{} ({}", first_word, inner_words);
        }

        let final_word = self.expand(words.last().unwrap()).capitalize();

        if words.len() > 3 {
            inner_words = self.handle_initials(words[1..words.len() - 2].join("-").as_str());
        }

        self.in_brackets = false;
        format!("{} ({}) {}", first_word, inner_words, final_word)
    }

    fn handle_short_dash(&self, string: &str, index: usize, count: usize) -> String {
        let words: Vec<_> = string.split("-").collect();
        words
            .iter()
            .map(|w| self.smart_capitalize(w, index, count))
            .collect::<Vec<String>>()
            .join("-")
    }

    fn smart_capitalize(&self, word: &str, index: usize, count: usize) -> String {
        println!("smart capsing {} index={} count={}", word, index, count);
        let chars: Vec<_> = word.chars().collect();
        let lowercase_word = word.to_lowercase();

        if chars.len() > 2 && chars[0].is_alphabetic() && chars[1] == '.' {
            // println!("initialism case");
            // initialism
            word.to_string()
        } else if WORDS.no_caps.contains(&lowercase_word) && index >= 1 && index <= count - 2 {
            lowercase_word.to_string()
        } else if WORDS.all_caps.contains(&lowercase_word) {
            // println!("all caps case");
            word.to_uppercase().to_string()
        } else if chars.iter().all(|c| c.is_uppercase() || c.is_numeric()) {
            // println!("all caps case");
            word.to_string()
        } else {
            word.capitalize()
        }
    }

    fn join_up(&self, words: &Vec<String>) -> String {
        let mut ret = words.join(" ");
        if self.in_brackets {
            ret.push(')');
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_title() {
        let mut tm = TagMaker::new();
        assert_eq!("Blue Bell Knoll", tm.title_from("blue_bell_knoll"));
        assert_eq!(
            "The Boy with the Arab Strap",
            tm.title_from("the_boy_with_the_arab_strap")
        );

        assert_eq!("Enemies EP", tm.title_from("enemies_ep"));
        assert_eq!("I Want to Wake Up", tm.title_from("i_want_to_wake_up"));

        // //   fn test_title_contraction() {
        assert_eq!("Don't Go", tm.title_from("dont_go"));
        assert_eq!(
            "You Can't Hold What You Haven't Got in Your Hand",
            tm.title_from("you_cant_hold_what_you_havent_got_in_your_hand")
        );
        //   }

        //   fn test_title_inches() {
        assert_eq!(
            "I Feel Love (12\" Mix)",
            tm.title_from("i_feel_love--12inch_mix")
        );
        assert_eq!("Fugitive (7\" Mix)", tm.title_from("fugitive--7inch_mix"));
        //   }

        //   fn test_title_hyphen() {
        assert_eq!("When-Never", tm.title_from("when-never"));
        assert_eq!(
            "Who-What and Maybe-Not",
            tm.title_from("who-what_and_maybe-not")
        );
        assert_eq!("Tick-Tock-Tick-Tock", tm.title_from("tick-tock-tick-tock"));
        //   }

        //   fn test_title_brackets_in_middle() {
        assert_eq!(
            "This Is (Almost) Too Easy",
            tm.title_from("this_is--almost--too_easy")
        );
        assert_eq!(
            "Can We Make It (Just a Little Bit) Harder",
            tm.title_from("can_we_make_it--just_a_little_bit--harder")
        );
        assert_eq!(
            "Title }ing In (Bracketed Words)",
            tm.title_from("title_}ing_in--bracketed_words")
        );
        assert_eq!(
            "Two (Lots) Of Brackets (Is Tricky)",
            tm.title_from("two--lots--of_brackets--is_tricky")
        );
        assert_eq!(
            "Variations 3 (Canon on the Unison)",
            tm.title_from("variations_3--canon_on_the_unison")
        );
        //   }

        //   fn test_title_brackets_at_}() {
        assert_eq!(
            "Suburbia (The Full Horror)",
            tm.title_from("suburbia--the_full_horror")
        );

        assert_eq!(
            "Om Mani Padme Hum 3 (Piano Version)",
            tm.title_from("om_mani_padme_hum_3--piano_version")
        );
        assert_eq!("Drumming (Part III)", tm.title_from("drumming--part_iii"));
        //   }

        // fn test_title_initials() {
        assert_eq!("C.R.E.E.P.", tm.title_from("c-r-e-e-p"));
        assert_eq!("The N.W.R.A.", tm.title_from("the_n-w-r-a"));
        assert_eq!("W.M.C. Blob 59", tm.title_from("w-m-c_blob_59"));
        // }

        //   fn test_initials_in_brackets() {
        assert_eq!(
            "The (I.N.I.T.I.A.L.S.) In Brackets",
            tm.title_from("the--i-n-i-t-i-a-l-s--in_brackets")
        );
        //   }
    }

    #[test]
    fn test_artist() {
        let mut tm = TagMaker::new();
        //   fn test_artist() {
        assert_eq!("Stereolab", tm.artist_from("stereolab"));
        assert_eq!("Royal Trux", tm.artist_from("royal_trux"));
        assert_eq!(
            "Jeffrey Lewis and The Bolts",
            tm.artist_from("jeffrey_lewis_and_the_bolts")
        );
        assert_eq!(
            "Someone featuring Someone Else",
            tm.artist_from("someone_ft_someone_else")
        );
        assert_eq!("R.E.M.", tm.artist_from("r-e-m"));
        assert_eq!("M.J. Hibbett", tm.artist_from("m-j_hibbett"));
        // assert_eq!("ABBA", tm.artist_from("abba"));
        // assert_eq!("Add N to (X)", tm.artist_from("add_n_to_x"));
        //   }
    }

    //   fn test_album() {
    #[test]
    fn test_album() {
        let mut tm = TagMaker::new();
        assert_eq!("Spiderland", tm.album_from("spiderland"));
        assert_eq!("Mars Audiac Quintet", tm.album_from("mars_audiac_quintet"));
        assert_eq!(
            "The Decline and Fall of Heavenly",
            tm.album_from("the_decline_and_fall_of_heavenly")
        );
    }

    //   fn test_genre() {
    //     assert_eq!("Noise", genre_from("noise"));
    //     assert_eq!("Noise", genre_from("Noise "));
    //     assert_eq!("Hip-Hop", genre_from("Hip-Hop"));
    //     assert_eq!("Rock and Roll", genre_from("rock AND roll"));
    //   }

    //   fn test_t_num() {
    #[test]
    fn test_t_num() {
        let mut tm = TagMaker::new();
        assert_eq!(1, tm.t_num_from("01"));
        assert_eq!(39, tm.t_num_from("39"));
    }
    //   }
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
