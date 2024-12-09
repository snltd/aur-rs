use crate::utils::tag_maker::TagMaker;
use crate::utils::words::Words;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TagValidator<'a> {
    current_year: i32,
    tag_maker: TagMaker<'a>,
}

impl<'a> TagValidator<'a> {
    pub fn new(words: &'a Words) -> Self {
        TagValidator {
            tag_maker: TagMaker::new(words),
            current_year: this_year(),
        }
    }

    pub fn validate_artist(&self, tag: &str) -> bool {
        has_nothing_forbidden(tag)
    }

    pub fn validate_title(&self, tag: &str) -> bool {
        // sanitised(tag) == self.tag_maker.title_from(tag) &&
        self.validate_artist(tag)
    }

    pub fn validate_album(&self, tag: &str) -> bool {
        self.validate_artist(tag)
    }

    pub fn validate_t_num(&self, tag: &str) -> bool {
        if !(1..=2).contains(&tag.len()) || tag.starts_with('0') {
            return false;
        }

        tag.chars().all(|c| c.is_numeric())
    }

    pub fn validate_year(&self, tag: &str) -> bool {
        if tag.len() != 4 {
            return false;
        }

        match tag.parse::<i32>() {
            Ok(year) => (1938..=self.current_year).contains(&year),
            Err(_) => false,
        }
    }

    pub fn validate_genre(&self, tag: &str) -> bool {
        tag == self.tag_maker.genre_from(tag) && !tag.is_empty()
    }
}

// It's bad if a string starts or finishes with whitespace, contains consecutive whitespace, or
// other forbidden characters.
fn has_nothing_forbidden(string: &str) -> bool {
    if string.is_empty() {
        return false;
    }

    let last_index = if string.len() > 2 {
        string.len() - 2
    } else {
        string.len()
    };

    let chars: Vec<char> = string.chars().collect();

    for (i, c) in chars.windows(2).enumerate() {
        if c[0] == '&' || c[0] == ';' || c[0] == '\u{2019}' {
            return false;
        }

        if (c[0].is_whitespace() && (i == 0 || c[1].is_whitespace()))
            || (c[1].is_whitespace() && i == last_index)
        {
            return false;
        }

        if c[0] == ',' && c[1].is_alphabetic() {
            return false;
        }
    }

    true
}

// // We ignore certain punctuation in titles. We have no way of encoding things like commas and
// // question marks in our filename schema. This is a best-guess thing. It can't possibly be
// // perfect.
// //
// fn sanitised(string: &str) -> String {
//     string
//         .replace([',', '’'], "")
//         .replace("Feat. ", "Feat")
//         .replace("' ", " ")
//         .trim_end_matches(['?', '!', '\''])
//         .to_string()
// }

fn this_year() -> i32 {
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seconds_since_epoch = duration_since_epoch.as_secs();
    1970 + (seconds_since_epoch / (60 * 60 * 24 * 365)) as i32
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::sample_config;

    #[test]
    fn test_validate_artist_album_and_title() {
        let words = Words::new(&sample_config());
        let tv = TagValidator::new(&words);
        assert!(tv.validate_artist("!!!"));
        assert!(tv.validate_artist("Broadcast"));
        assert!(tv.validate_artist("Simon and Garfunkel"));
        assert!(tv.validate_artist("Simon And Garfunkel"));
        assert!(tv.validate_artist("Comma, Space"));
        assert!(tv.validate_artist("100,000 Fireflies"));
        assert!(!tv.validate_artist("Broadcast "));
        assert!(!tv.validate_artist("Simon and  Garfunkel"));
        assert!(!tv.validate_artist("Broadcast;Broadcast"));
        assert!(!tv.validate_artist("Simon & Garfunkel"));
        assert!(!tv.validate_artist(""));
        assert!(!tv.validate_title("Cybele’s Reverie"));
        assert!(!tv.validate_title("Comma,No Space"));
    }

    #[test]
    fn test_validate_year() {
        let words = Words::new(&sample_config());
        let tv = TagValidator::new(&words);
        assert!(tv.validate_year("1994"));
        assert!(!tv.validate_year(&(this_year() + 2).to_string()));
        assert!(!tv.validate_year("1930"));
        assert!(!tv.validate_year(""));
        assert!(!tv.validate_year("1996/2020"));
        assert!(!tv.validate_year("1989 02 03"));
    }

    //   def test_artist_strict
    //     refute @strict.artist('Singer and the Band')
    //   end

    #[test]
    fn test_validate_title() {
        let words = Words::new(&sample_config());
        let tv = TagValidator::new(&words);
        assert!(tv.validate_title("File for Test"));
        assert!(!tv.validate_title("File,with Bad Title"));
    }

    #[test]
    fn test_validate_t_num() {
        let words = Words::new(&sample_config());
        let tv = TagValidator::new(&words);
        assert!(tv.validate_t_num("1"));
        assert!(tv.validate_t_num("10"));
        assert!(!tv.validate_t_num("01"));
        assert!(!tv.validate_t_num("-1"));
        assert!(!tv.validate_t_num(""));
        assert!(!tv.validate_t_num("0"));
        assert!(!tv.validate_t_num("1/14"));
        assert!(!tv.validate_t_num("01/14"));
        assert!(!tv.validate_t_num("1 (disc 1)"));
    }

    #[test]
    fn test_validate_genre() {
        let words = Words::new(&sample_config());
        let tv = TagValidator::new(&words);
        assert!(tv.validate_genre("Alternative"));
        assert!(tv.validate_genre("Noise"));
        assert!(tv.validate_genre("Hip-Hop"));
        assert!(tv.validate_genre("Folk Rock"));
        assert!(tv.validate_genre("Rock and Roll"));
        assert!(!tv.validate_genre("Folk rock"));
        assert!(!tv.validate_genre("Hip-hop"));
        assert!(!tv.validate_genre("Folk/Rock"));
        assert!(!tv.validate_genre("noise"));
        assert!(!tv.validate_genre(""));
    }
}
