use crate::utils::string::Capitalize;
use crate::utils::words::Words;

/// Retitler takes an existing tag and tries to bring it more in line with our tagging rules.
/// It looks superficially similar to TagMaker, but they take sufficiently different inputs to
/// warrant different implementations.
pub struct Retitler<'a> {
    words: &'a Words,
}

const PLACEHOLDER: &str = "xxPLACEHOLDERxx";

impl<'a> Retitler<'a> {
    pub fn new(words: &'a Words) -> Self {
        Self { words }
    }

    pub fn retitle(&self, old_title: &str) -> String {
        let mut words: Vec<_> = old_title.split_whitespace().collect();
        words.splice(0..0, [PLACEHOLDER]);
        words.push(PLACEHOLDER);

        let ret: Vec<_> = words
            .windows(3)
            .filter_map(|window| {
                if let [before, word, after] = window {
                    Some(if *before == PLACEHOLDER || *after == PLACEHOLDER {
                        self.titlecase(word, "/", false)
                    } else {
                        self.titlecase(word, before, false)
                    })
                } else {
                    None
                }
            })
            .collect();

        ret.join(" ")
    }

    fn titlecase(&self, word: &str, previous_word: &str, run_together: bool) -> String {
        if word.is_empty() {
            return word.to_string();
        }
        let chars: Vec<_> = word.chars().collect();

        if !chars[0].is_alphanumeric() {
            return self.start_with_nonword(word);
        } else if word.ends_with([':', '=', ')']) {
            return self.follow_punct(word);
        }

        for sep in ['=', '-', '+', '/', ':', '.'] {
            if word.contains(sep) {
                return self.contains_sep(word, sep);
            }
        }

        if self.ignorecase(word) {
            return word.to_string();
        }

        let stripped_word = self.downcase_string(word);

        if self.is_upcase(word, &stripped_word) {
            return word.to_uppercase();
        }

        if self.is_downcase(&stripped_word, previous_word, run_together) {
            return word.to_lowercase();
        }

        word.capitalize()
    }

    fn start_with_nonword(&self, word: &str) -> String {
        let mut chars = word.chars();
        let mut nonword_prefix = String::new();

        while let Some(c) = chars.next() {
            if c.is_alphanumeric() {
                nonword_prefix.push(c);
                break;
            }
            nonword_prefix.push(c);
        }

        let rest: String = chars.collect();
        format!("{}{}", nonword_prefix, self.titlecase(&rest, "/", false))
    }

    fn follow_punct(&self, word: &str) -> String {
        let mut chars = word.chars().peekable();
        let mut result = String::new();
        let mut first_word = String::new();

        while let Some(&c) = chars.peek() {
            if c.is_alphanumeric() {
                first_word.push(c);
                chars.next();
            } else {
                break;
            }
        }

        result.push_str(&self.titlecase(&first_word, "/", false));

        if let Some(c) = chars.next() {
            result.push(c);
        }

        result.extend(chars);
        result
    }

    fn contains_sep(&self, word: &str, sep: char) -> String {
        word.split(sep)
            .enumerate()
            .map(|(i, w)| self.titlecase(w, "/", i > 0))
            .collect::<Vec<_>>()
            .join(&sep.to_string())
    }

    fn ignorecase(&self, word: &str) -> bool {
        self.words.ignore_case.contains(&word.to_lowercase())
    }

    fn is_downcase(&self, word: &str, previous_word: &str, run_together: bool) -> bool {
        self.words.no_caps.contains(&word.to_lowercase())
            && (run_together || !previous_word.ends_with(['[', ':', '-', '/', '?', '!']))
    }

    fn is_upcase(&self, word: &str, stripped_word: &str) -> bool {
        (self.words.all_caps.contains(stripped_word))
            || (word.chars().count() == 3 && word.chars().nth(1) == Some('.'))
            || (word.chars().count() == 2 && word.chars().nth(1) == Some(','))
    }

    fn downcase_string(&self, word: &str) -> String {
        word.chars().filter(|c| c.is_alphanumeric()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::sample_config;
    #[test]
    fn test_retitle() {
        let words = Words::new(&sample_config());
        let rt = Retitler::new(&words);
        assert_eq!("Original Title", rt.retitle("Original Title"));
        assert_eq!("Fix the Article", rt.retitle("Fix The Article"));
        assert_eq!(
            "One of the Ones Where We Fix a Word or Two",
            rt.retitle("One Of The Ones Where We Fix A Word Or Two")
        );
        assert_eq!(
            "This, that, and, Yes, the Other",
            rt.retitle("This, That, And, Yes, The Other")
        );
        assert_eq!("It is is It", rt.retitle("It Is Is It"));
        assert_eq!(
            "A: The Thing of Things",
            rt.retitle("A: The Thing Of Things")
        );
        assert_eq!(
            "One Thing / And the Other",
            rt.retitle("One Thing / And The Other")
        );
        assert_eq!("It is Narrow Here", rt.retitle("It Is Narrow here"));
        assert_eq!(
            "The Song of the Nightingale / The Firebird Suite / The Rite of Spring",
            rt.retitle("The Song Of The Nightingale / The Firebird Suite / The Rite of Spring")
        );
        assert_eq!("P.R.O.D.U.C.T.", rt.retitle("p.r.o.d.u.c.t."));
        assert_eq!("Aikea-Guinea", rt.retitle("aikea-guinea"));
        assert_eq!("Kill-a-Man", rt.retitle("kill-a-man"));
        assert_eq!("Master=Dik", rt.retitle("Master=dik"));
        assert_eq!("Fixed::Content", rt.retitle("fixed::content"));
        assert_eq!("Itchy+Scratchy", rt.retitle("itchy+scratcHy"));
    }
}
