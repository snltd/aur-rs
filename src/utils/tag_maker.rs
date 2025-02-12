use super::metadata::AurMetadata;
use super::string::ToLowerAlnums;
use crate::utils::string::Capitalize;
use crate::utils::words::Words;
use anyhow::anyhow;

type InBrackets = bool;

/// TagMaker makes roughly correct tags given snake_cased_strings. Obviously it can't guess much
/// in the way of punctuation, but it handles quite a lot of odd cases.
pub struct TagMaker<'a> {
    words: &'a Words,
    force: bool,
}

pub struct TagMakerAllTags {
    pub artist: String,
    pub title: String,
    pub album: String,
    pub t_num: u32,
}

impl<'a> TagMaker<'a> {
    pub fn new(words: &'a Words, force: bool) -> Self {
        Self { words, force }
    }

    pub fn all_tags_from(&self, info: &AurMetadata) -> anyhow::Result<TagMakerAllTags> {
        let path = info.path.canonicalize()?;
        let album_dir = match path.parent() {
            Some(dir) => dir,
            None => {
                return Err(anyhow!(
                    "could not get album directory for {}",
                    info.path.display()
                ))
            }
        };

        let album_dir_name = match album_dir.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => {
                return Err(anyhow!(
                    "could not get album directory name for {}",
                    info.path.display()
                ))
            }
        };

        let fname_chunks: Vec<_> = info.filename.split('.').collect();

        if fname_chunks.len() != 4 {
            return Err(anyhow!(
                "Expected four parts in track name '{}': got {}",
                info.filename,
                fname_chunks.len()
            ));
        }

        let album_chunks: Vec<_> = album_dir_name.split('.').collect();
        let album_name: &str;

        if album_chunks.len() == 2 {
            album_name = album_chunks[1];
        } else if self.force {
            album_name = "";
        } else {
            return Err(anyhow!(
                "Expected two parts in album name '{}': got {}. Use -f to force empty album tag",
                album_dir_name,
                album_chunks.len()
            ));
        }

        let ret = TagMakerAllTags {
            artist: self.artist_from(fname_chunks[1]),
            title: self.title_from(fname_chunks[2]),
            album: self.album_from(album_name),
            t_num: self.t_num_from(fname_chunks[0]),
        };

        Ok(ret)
    }

    pub fn title_from(&self, string: &str) -> String {
        let mut in_brackets = false;
        let words: Vec<_> = string.split(['_', ' ']).collect();
        let mut bits: Vec<String> = Vec::new();
        let word_count = words.len();

        for (i, word) in words.iter().enumerate() {
            let (new_word, ib) = self.handle_string(word, i, word_count, in_brackets);
            bits.push(new_word);
            in_brackets = ib;
        }

        self.join_up(&bits, in_brackets)
    }

    pub fn artist_from(&self, string: &str) -> String {
        self.title_from(string).replace("and the ", "and The ")
    }

    pub fn album_from(&self, string: &str) -> String {
        self.title_from(string)
    }

    #[allow(dead_code)]
    pub fn genre_from(&self, string: &str) -> String {
        self.title_from(string).trim().to_string()
    }

    pub fn t_num_from(&self, number: &str) -> u32 {
        number.to_string().parse::<u32>().unwrap_or(0)
    }

    fn handle_string(
        &self,
        string: &str,
        index: usize,
        count: usize,
        in_brackets: InBrackets,
    ) -> (String, InBrackets) {
        let chars: Vec<_> = string.chars().collect();

        if chars.len() >= 3
            && chars[0].is_alphabetic()
            && chars[1] == '-'
            && chars[2].is_alphabetic()
        {
            (self.handle_initials(string), in_brackets)
        } else if string.contains("--") {
            self.handle_long_dash(string, in_brackets)
        } else if string.contains("-") {
            (self.handle_short_dash(string, index, count), in_brackets)
        } else {
            (
                self.smart_capitalize(&self.expand(string), index, count),
                in_brackets,
            )
        }
    }

    fn handle_initials(&self, string: &str) -> String {
        let mut ret = string.to_uppercase().replace("-", ".");
        ret.push('.');
        ret
    }

    fn expand(&self, word: &str) -> String {
        self.words
            .expand
            .get(word)
            .map(|s| s.as_str())
            .unwrap_or(word)
            .to_string()
    }

    fn handle_long_dash(&self, string: &str, in_brackets: InBrackets) -> (String, InBrackets) {
        let words: Vec<_> = string.split("--").collect();

        if in_brackets {
            self.close_brackets(words)
        } else {
            self.open_brackets(words, in_brackets)
        }
    }

    fn handle_short_dash(&self, string: &str, index: usize, count: usize) -> String {
        let words: Vec<_> = string.split("-").collect();
        words
            .iter()
            .map(|w| self.smart_capitalize(w, index, count))
            .collect::<Vec<String>>()
            .join("-")
    }

    fn open_brackets(&self, words: Vec<&str>, in_brackets: InBrackets) -> (String, InBrackets) {
        let first_word = self.expand(words[0]).capitalize();
        let (mut inner_words, in_brackets) = self.handle_string(words[1], 0, 0, in_brackets);

        if words.len() <= 2 {
            return (format!("{} ({}", first_word, inner_words), true);
        }

        let final_word = self.expand(words.last().unwrap()).capitalize();

        if words.len() > 3 {
            inner_words = self.handle_initials(&words[1..words.len() - 2].join("-"));
        }

        (
            format!("{} ({}) {}", first_word, inner_words, final_word),
            in_brackets,
        )
    }

    fn close_brackets(&self, words: Vec<&str>) -> (String, InBrackets) {
        (
            format!(
                "{}) {}",
                self.expand(words[0]).capitalize(),
                self.expand(words[1]).capitalize()
            ),
            false,
        )
    }

    fn smart_capitalize(&self, word: &str, index: usize, count: usize) -> String {
        let chars: Vec<_> = word.chars().collect();
        let lowercase_word = word.to_lower_alnums();

        if chars.len() > 2 && chars[0].is_alphabetic() && chars[1] == '.' {
            word.to_string()
        } else if self.words.no_caps.contains(&lowercase_word) && index >= 1 && index <= count - 2 {
            word.to_lowercase().to_string()
        } else if self.words.all_caps.contains(&lowercase_word) {
            word.to_uppercase().to_string()
        } else if chars.iter().all(|c| c.is_uppercase() || c.is_numeric()) {
            word.to_string()
        } else {
            word.capitalize()
        }
    }

    fn join_up(&self, words: &[String], in_brackets: InBrackets) -> String {
        let mut ret = words.join(" ");

        if in_brackets {
            ret.push(')');
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::sample_config;

    #[test]
    fn test_title_from() {
        let words = Words::new(&sample_config());
        let tm = TagMaker::new(&words, false);

        assert_eq!("Blue Bell Knoll", tm.title_from("blue_bell_knoll"));
        assert_eq!(
            "The Boy with the Arab Strap",
            tm.title_from("the_boy_with_the_arab_strap")
        );
        assert_eq!("Enemies EP", tm.title_from("enemies_ep"));
        assert_eq!("I Want to Wake Up", tm.title_from("i_want_to_wake_up"));
        assert_eq!("Don't Go", tm.title_from("dont_go"));
        assert_eq!(
            "You Can't Hold What You Haven't Got in Your Hand",
            tm.title_from("you_cant_hold_what_you_havent_got_in_your_hand")
        );
        assert_eq!(
            "I Feel Love (12\" Mix)",
            tm.title_from("i_feel_love--12inch_mix")
        );
        assert_eq!("Fugitive (7\" Mix)", tm.title_from("fugitive--7inch_mix"));
        assert_eq!("When-Never", tm.title_from("when-never"));
        assert_eq!(
            "Who-What and Maybe-Not",
            tm.title_from("who-what_and_maybe-not")
        );
        assert_eq!("Tick-Tock-Tick-Tock", tm.title_from("tick-tock-tick-tock"));
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
        assert_eq!(
            "Suburbia (The Full Horror)",
            tm.title_from("suburbia--the_full_horror")
        );

        assert_eq!(
            "Om Mani Padme Hum 3 (Piano Version)",
            tm.title_from("om_mani_padme_hum_3--piano_version")
        );
        assert_eq!("Drumming (Part III)", tm.title_from("drumming--part_iii"));
        assert_eq!("C.R.E.E.P.", tm.title_from("c-r-e-e-p"));
        assert_eq!("The N.W.R.A.", tm.title_from("the_n-w-r-a"));
        assert_eq!("W.M.C. Blob 59", tm.title_from("w-m-c_blob_59"));
        assert_eq!(
            "The (I.N.I.T.I.A.L.S.) In Brackets",
            tm.title_from("the--i-n-i-t-i-a-l-s--in_brackets")
        );
        assert_eq!("Stereolab", tm.artist_from("stereolab"));
        assert_eq!("Royal Trux", tm.artist_from("royal_trux"));
        assert_eq!(
            "Jeffrey Lewis and The Bolts",
            tm.artist_from("jeffrey_lewis_and_the_bolts")
        );
        assert_eq!(
            "Someone feat. Someone Else",
            tm.artist_from("someone_ft_someone_else")
        );
        assert_eq!("R.E.M.", tm.artist_from("r-e-m"));
        assert_eq!("M.J. Hibbett", tm.artist_from("m-j_hibbett"));

        // The next two use the sample config
        assert_eq!("ABBA", tm.artist_from("abba"));
        // assert_eq!("Add N to (X)", tm.artist_from("add_n_to_"));

        assert_eq!("Spiderland", tm.album_from("spiderland"));
        assert_eq!("Mars Audiac Quintet", tm.album_from("mars_audiac_quintet"));
        assert_eq!(
            "The Decline and Fall of Heavenly",
            tm.album_from("the_decline_and_fall_of_heavenly")
        );
        assert_eq!("Noise", tm.genre_from("noise"));
        assert_eq!("Noise", tm.genre_from("Noise "));
        assert_eq!("Hip-Hop", tm.genre_from("Hip-Hop"));
        assert_eq!("Rock and Roll", tm.genre_from("rock AND roll"));
        assert_eq!(1, tm.t_num_from("01"));
        assert_eq!(39, tm.t_num_from("39"));
    }
}
