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
            all_caps.extend(more_words.into_iter().cloned());
        }

        if let Some(more_words) = config.get_words_no_caps() {
            no_caps.extend(more_words.into_iter().cloned());
        }

        if let Some(more_words) = config.get_words_ignore_case() {
            ignore_case.extend(more_words.into_iter().cloned());
        }

        if let Some(more_words) = config.get_words_expand() {
            expand.extend(
                more_words
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string())),
            );
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
        "cd".to_string(),
        "diy".to_string(),
        "dvd".to_string(),
        "ep".to_string(),
        "ii".to_string(),
        "iii".to_string(),
        "iv".to_string(),
        "ix".to_string(),
        "lp".to_string(),
        "ok".to_string(),
        "ps".to_string(),
        "uk".to_string(),
        "usa".to_string(),
        "vi".to_string(),
        "vii".to_string(),
        "viii".to_string(),
        "xi".to_string(),
        "xii".to_string(),
        "xiii".to_string(),
        "xiv".to_string(),
        "xix".to_string(),
        "xv".to_string(),
        "xvi".to_string(),
        "xvii".to_string(),
        "xviii".to_string(),
        "xx".to_string(),
        "xxi".to_string(),
        "xxv".to_string(),
        "xxx".to_string(),
    ])
}

fn no_caps() -> HashSet<String> {
    HashSet::from([
        "a".to_string(),
        "am".to_string(),
        "an".to_string(),
        "and".to_string(),
        "are".to_string(),
        "as".to_string(),
        "at".to_string(),
        "au".to_string(),
        "by".to_string(),
        "ce".to_string(),
        "dans".to_string(),
        "de".to_string(),
        "des".to_string(),
        "du".to_string(),
        "es".to_string(),
        "est".to_string(),
        "et".to_string(),
        "feat".to_string(),
        "featuring".to_string(),
        "for".to_string(),
        "from".to_string(),
        "in".to_string(),
        "into".to_string(),
        "is".to_string(),
        "it".to_string(),
        "its".to_string(),
        "la".to_string(),
        "le".to_string(),
        "ne".to_string(),
        "nor".to_string(),
        "o'clock".to_string(),
        "of".to_string(),
        "off".to_string(),
        "on".to_string(),
        "onto".to_string(),
        "or".to_string(),
        "out".to_string(),
        "pas".to_string(),
        "per".to_string(),
        "se".to_string(),
        "so".to_string(),
        "te".to_string(),
        "than".to_string(),
        "that".to_string(),
        "the".to_string(),
        "till".to_string(),
        "to".to_string(),
        "too".to_string(),
        "un".to_string(),
        "une".to_string(),
        "via".to_string(),
        "vs".to_string(),
        "when".to_string(),
        "with".to_string(),
    ])
}

fn ignore_case() -> HashSet<String> {
    HashSet::from(["x".to_string()])
}

fn expand() -> HashMap<String, String> {
    HashMap::from([
        ("12inch".to_string(), "12\"".to_string()),
        ("7inch".to_string(), "7\"".to_string()),
        ("--".to_string(), " - ".to_string()),
        ("&".to_string(), "and".to_string()),
        ("aint".to_string(), "ain't".to_string()),
        ("cant".to_string(), "can't".to_string()),
        ("couldnt".to_string(), "couldn't".to_string()),
        ("didnt".to_string(), "didn't".to_string()),
        ("dj".to_string(), "DJ".to_string()),
        ("doesnt".to_string(), "doesn't".to_string()),
        ("dont".to_string(), "don't".to_string()),
        ("etc".to_string(), "etc.".to_string()),
        ("ft".to_string(), "featuring".to_string()),
        ("hes".to_string(), "he's".to_string()),
        ("havent".to_string(), "haven't".to_string()),
        ("ive".to_string(), "I've".to_string()),
        ("im".to_string(), "I'm".to_string()),
        ("isnt".to_string(), "isn't".to_string()),
        ("its".to_string(), "it's".to_string()),
        ("lets".to_string(), "let's".to_string()),
        ("n".to_string(), "'n'".to_string()),
        ("shes".to_string(), "she's".to_string()),
        ("thats".to_string(), "that's".to_string()),
        ("theres".to_string(), "there's".to_string()),
        ("wasnt".to_string(), "wasn't".to_string()),
        ("weve".to_string(), "we've".to_string()),
        ("whats".to_string(), "what's".to_string()),
        ("whos".to_string(), "who's".to_string()),
        ("wont".to_string(), "won't".to_string()),
        ("wouldnt".to_string(), "wouldn't".to_string()),
        ("youll".to_string(), "you'll".to_string()),
        ("youre".to_string(), "you're".to_string()),
        ("youve".to_string(), "you've".to_string()),
    ])
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::config::load_config;
    use crate::utils::spec_helper::fixture;

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
