use crate::utils::config::Config;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Words {
    pub no_caps: HashSet<String>,
    pub all_caps: HashSet<String>,
    pub ignore_case: HashSet<String>,
    pub expand: HashMap<String, String>,
}

impl Words {
    pub fn new(config: &Config) -> Self {
        let mut all_caps = all_caps();
        let mut no_caps = no_caps();
        let mut ignore_case = ignore_case();
        let mut expand = expand();

        if let Some(more_words) = config.get_words_all_caps() {
            all_caps.extend(more_words.iter().cloned());
        }

        if let Some(more_words) = config.get_words_no_caps() {
            no_caps.extend(more_words.iter().cloned());
        }

        if let Some(more_words) = config.get_words_ignore_case() {
            ignore_case.extend(more_words.iter().cloned());
        }

        if let Some(more_words) = config.get_words_expand() {
            expand.extend(more_words.iter().map(|(k, v)| (k.to_owned(), v.to_owned())));
        }

        Self {
            no_caps,
            all_caps,
            ignore_case,
            expand,
        }
    }
}

fn all_caps() -> HashSet<String> {
    HashSet::from([
        "cd".to_owned(),
        "diy".to_owned(),
        "dj".to_owned(),
        "dvd".to_owned(),
        "ep".to_owned(),
        "ii".to_owned(),
        "iii".to_owned(),
        "iv".to_owned(),
        "ix".to_owned(),
        "lp".to_owned(),
        "ok".to_owned(),
        "ps".to_owned(),
        "uk".to_owned(),
        "usa".to_owned(),
        "vi".to_owned(),
        "vii".to_owned(),
        "viii".to_owned(),
        "xi".to_owned(),
        "xii".to_owned(),
        "xiii".to_owned(),
        "xiv".to_owned(),
        "xix".to_owned(),
        "xv".to_owned(),
        "xvi".to_owned(),
        "xvii".to_owned(),
        "xviii".to_owned(),
        "xx".to_owned(),
        "xxi".to_owned(),
        "xxv".to_owned(),
        "xxx".to_owned(),
    ])
}

fn no_caps() -> HashSet<String> {
    HashSet::from([
        "a".to_owned(),
        "am".to_owned(),
        "an".to_owned(),
        "and".to_owned(),
        "are".to_owned(),
        "as".to_owned(),
        "at".to_owned(),
        "au".to_owned(),
        "by".to_owned(),
        "ce".to_owned(),
        "dans".to_owned(),
        "de".to_owned(),
        "des".to_owned(),
        "du".to_owned(),
        "es".to_owned(),
        "est".to_owned(),
        "et".to_owned(),
        "feat".to_owned(),
        "for".to_owned(),
        "from".to_owned(),
        "in".to_owned(),
        "into".to_owned(),
        "is".to_owned(),
        "it".to_owned(),
        "its".to_owned(),
        "la".to_owned(),
        "le".to_owned(),
        "ne".to_owned(),
        "nor".to_owned(),
        "o'clock".to_owned(),
        "of".to_owned(),
        "off".to_owned(),
        "on".to_owned(),
        "onto".to_owned(),
        "or".to_owned(),
        "out".to_owned(),
        "pas".to_owned(),
        "per".to_owned(),
        "se".to_owned(),
        "so".to_owned(),
        "te".to_owned(),
        "than".to_owned(),
        "that".to_owned(),
        "the".to_owned(),
        "till".to_owned(),
        "to".to_owned(),
        "too".to_owned(),
        "un".to_owned(),
        "une".to_owned(),
        "via".to_owned(),
        "vs".to_owned(),
        "when".to_owned(),
        "with".to_owned(),
    ])
}

fn ignore_case() -> HashSet<String> {
    HashSet::from(["x".to_owned()])
}

fn expand() -> HashMap<String, String> {
    HashMap::from([
        ("12inch".to_owned(), "12\"".to_owned()),
        ("7inch".to_owned(), "7\"".to_owned()),
        ("--".to_owned(), " - ".to_owned()),
        ("&".to_owned(), "and".to_owned()),
        ("aint".to_owned(), "ain't".to_owned()),
        ("cant".to_owned(), "can't".to_owned()),
        ("couldnt".to_owned(), "couldn't".to_owned()),
        ("didnt".to_owned(), "didn't".to_owned()),
        ("dj".to_owned(), "DJ".to_owned()),
        ("doesnt".to_owned(), "doesn't".to_owned()),
        ("dont".to_owned(), "don't".to_owned()),
        ("etc".to_owned(), "etc.".to_owned()),
        ("featuring".to_owned(), "feat.".to_owned()),
        ("ft".to_owned(), "feat.".to_owned()),
        ("hes".to_owned(), "he's".to_owned()),
        ("havent".to_owned(), "haven't".to_owned()),
        ("ive".to_owned(), "I've".to_owned()),
        ("im".to_owned(), "I'm".to_owned()),
        ("isnt".to_owned(), "isn't".to_owned()),
        ("its".to_owned(), "it's".to_owned()),
        ("lets".to_owned(), "let's".to_owned()),
        ("n".to_owned(), "'n'".to_owned()),
        ("shes".to_owned(), "she's".to_owned()),
        ("thats".to_owned(), "that's".to_owned()),
        ("theres".to_owned(), "there's".to_owned()),
        ("wasnt".to_owned(), "wasn't".to_owned()),
        ("weve".to_owned(), "we've".to_owned()),
        ("whats".to_owned(), "what's".to_owned()),
        ("whos".to_owned(), "who's".to_owned()),
        ("wont".to_owned(), "won't".to_owned()),
        ("wouldnt".to_owned(), "wouldn't".to_owned()),
        ("youll".to_owned(), "you'll".to_owned()),
        ("youre".to_owned(), "you're".to_owned()),
        ("youve".to_owned(), "you've".to_owned()),
    ])
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::fixture;
    use crate::utils::config::load_config;

    #[test]
    fn test_words() {
        let config = load_config(&fixture("config/test.toml")).unwrap();
        let words = Words::new(&config);
        assert!(words.no_caps.contains("via"));
        assert!(words.all_caps.contains("dvd")); // hardcoded
        assert!(words.all_caps.contains("abba")); // config
        assert!(words.ignore_case.contains("x")); // hardcoded
        assert!(words.ignore_case.contains("mxbx")); // config
        assert!(words.expand.contains_key("shes")); // hardcoded
        assert!(words.expand.contains_key("add_n_to_x")); //config
    }
}
