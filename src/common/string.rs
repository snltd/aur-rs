// use unicode_normalization::UnicodeNormalization;
use unidecode::unidecode;

pub trait ToSafe {
    fn to_safe(&self) -> String;
}

impl ToSafe for String {
    // The rules for making a filename-safe string are to:
    //   - replace accented characters with basic Latin
    //   - make lowercase
    //   - remove anything that's not a letter, number, underscore or hyphen
    //   - things in brackets have the brackets removed and -- put in front and/or behind
    //   - turn all whitespace to a single underscore
    //   - turn '_-_' into a single hyphen
    //   - turn a hyphenated word into word-word, removing spaces
    fn to_safe(&self) -> String {
        let mut ret = String::new();
        let mut last_pushed = ' ';
        let mut double_dash = false;

        for c in unidecode(self.trim()).to_lowercase().chars() {
            match c {
                c if c.is_ascii_alphanumeric() || c == '_' || c == '-' => {
                    ret.push(c);
                    last_pushed = c;
                    double_dash = false;
                }
                ' ' if last_pushed != '_' && !double_dash => {
                    ret.push('_');
                    last_pushed = '_';
                }
                '.' => {
                    ret.push('-');
                    last_pushed = '-';
                }
                '/' | '>' if last_pushed != '-' => {
                    ret.push('-');
                    last_pushed = '-';
                }
                '+' => {
                    ret.push_str("plus");
                    last_pushed = 's';
                }
                '@' => {
                    ret.push_str("at");
                    last_pushed = 't';
                }
                '&' => {
                    ret.push_str("and");
                    last_pushed = 'd';
                }
                '(' | ')' | '[' | ']' => {
                    if last_pushed == '_' {
                        ret.pop();
                    }
                    ret.push_str("--");
                    last_pushed = '-';
                    double_dash = true;
                }
                _ => {}
            }
        }

        ret.trim_matches('-')
            .trim_matches('_')
            .replace("_-_", "--")
            .to_string()
    }
}

impl ToSafe for &str {
    fn to_safe(&self) -> String {
        self.to_string().to_safe()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_safe() {
        assert_eq!("".to_safe(), "");
        assert_eq!("basic".to_safe(), "basic");
        assert_eq!("Füxa".to_safe(), "fuxa");
        assert_eq!("R.E.M.".to_safe(), "r-e-m");
        assert_eq!(
            "20 Cases Suggestive of ...".to_safe(),
            "20_cases_suggestive_of"
        );
        assert_eq!("V/Vm".to_safe(), "v-vm");
        assert_eq!("Say \"Yes!\"".to_safe(), "say_yes");
        assert_eq!("...Baby One More Time".to_safe(), "baby_one_more_time");
        assert_eq!("simple-String".to_safe(), "simple-string");
        assert_eq!("Simple String".to_safe(), "simple_string");
        assert_eq!("Incident @ 23rd".to_safe(), "incident_at_23rd");
        assert_eq!("+2".to_safe(), "plus2");
        assert_eq!("Stripped String  ".to_safe(), "stripped_string");
        assert_eq!(
            "a long, complicated string-type-thing.".to_safe(),
            "a_long_complicated_string-type-thing"
        );
        assert_eq!("!|~~c*o*n*t*e*n*t~~;:".to_safe(), "content");
        assert_eq!(
            "Looking for Love (in the Hall of Mirrors)".to_safe(),
            "looking_for_love--in_the_hall_of_mirrors"
        );
        assert_eq!(
            "(You Gotta) Fight for Your Right (to Party!)".to_safe(),
            "you_gotta--fight_for_your_right--to_party"
        );
        assert_eq!(
            "this is (almost) too easy".to_safe(),
            "this_is--almost--too_easy"
        );
        assert_eq!(
            "I'm almost sure you're not...".to_safe(),
            "im_almost_sure_youre_not"
        );
    }
}
