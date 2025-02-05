use unidecode::unidecode;

pub trait Compacted {
    fn compacted(&self) -> String;
}

pub trait ToFilenameChunk {
    fn to_filename_chunk(&self) -> String;
    fn all_symbols(&self) -> String;
}

pub trait ReplaceLast {
    fn replace_last(&self, from: &str, to: &str) -> String;
}

pub trait Capitalize {
    fn capitalize(&self) -> String;
}

impl ToFilenameChunk for String {
    // The rules for making a filename-safe string are to:
    //   - replace accented characters with basic Latin
    //   - make lowercase
    //   - remove anything that's not a letter, number, underscore or hyphen
    //   - things in brackets have the brackets removed and -- put in front and/or behind
    //   - turn all whitespace to a single underscore
    //   - turn '_-_' into a single hyphen
    //   - turn a hyphenated word into word-word, removing spaces
    fn to_filename_chunk(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        if !self.chars().any(|c| c.is_ascii_alphanumeric()) {
            return self.all_symbols();
        }

        let mut double_dash = false;
        let mut builder: Vec<char> = Vec::new();

        let cleaned = self.trim().replace('£', "").to_lowercase();
        let input: Vec<char> = unidecode(&cleaned).chars().collect();
        let last_index = input.len() - 1;

        for (i, &c) in input.iter().enumerate().take(last_index + 1) {
            match c {
                '-' => {
                    if let Some(&last_pushed) = builder.last() {
                        if last_pushed == '_' {
                            builder.pop();
                            builder.extend(['-', '-']);
                            double_dash = true;
                        } else if last_pushed != '-' {
                            builder.push('-');
                        }
                    }
                }
                c if c.is_ascii_alphanumeric() => {
                    double_dash = false;
                    builder.push(c);
                }
                ' ' => {
                    if let Some(&last_pushed) = builder.last() {
                        if last_pushed == '-' && !double_dash {
                            builder.pop();
                        }
                        if last_pushed != '_' && !double_dash {
                            builder.push('_');
                        }
                    }
                }
                '.' => {
                    if let Some(&last_pushed) = builder.last() {
                        if last_pushed == '-' && !double_dash {
                            builder.pop();
                        }
                        if i < last_index && !double_dash && last_pushed != '_' {
                            builder.push('-');
                        }
                    }
                }
                '/' | '>' => {
                    if let Some(&last_pushed) = builder.last() {
                        if last_pushed == '_' {
                            builder.pop();
                        }
                        if last_pushed != '-' && i < last_index {
                            builder.extend(['-', '-']);
                            double_dash = true;
                        }
                    }
                }
                '"' => {
                    if let Some(&last_pushed) = builder.last() {
                        if last_pushed == '2' || last_pushed == '7' {
                            builder.extend(['_', 'i', 'n', 'c', 'h']);
                        }
                    }
                }
                '\'' => {
                    if let Some(&last_pushed) = builder.last() {
                        if last_pushed == 'o' || last_pushed == 'd' || last_pushed == 'l' {
                            match builder.get(builder.len().wrapping_sub(2)) {
                                Some('_') => builder.push('-'),
                                None => builder.push('-'),
                                Some(_) => (),
                            }
                        }
                    }
                }
                '(' | ')' | '[' | ']' | ':' => {
                    if let Some(&last_pushed) = builder.last() {
                        if last_pushed == '_' || last_pushed == '-' && !double_dash {
                            builder.pop();
                        }
                    }

                    if i < last_index && !double_dash && !builder.is_empty() {
                        builder.push('-');
                        builder.push('-');
                        double_dash = true;
                    }
                }
                '+' => {
                    if let Some(&next_char) = input.get(i + 1) {
                        if next_char == '-' {
                            builder.extend(['p', 'l', 'u', 's', '-', 'm', 'i', 'n', 'u', 's']);
                        } else if let Some(&last_pushed) = builder.last() {
                            if last_pushed == '_' && next_char == ' ' {
                                builder.extend(['a', 'n', 'd']);
                            } else {
                                builder.extend(['p', 'l', 'u', 's']);
                            }
                        } else {
                            builder.extend(['p', 'l', 'u', 's']);
                        }
                    } else {
                        builder.extend(['_', 'p', 'l', 'u', 's']);
                    }
                }
                '_' => {
                    if !builder.is_empty() && i < last_index {
                        builder.push('_');
                    }
                }
                '*' => {
                    if !builder.is_empty() && i < last_index {
                        builder.push('-');
                    }
                }
                '#' => {
                    if let Some(&last_pushed) = builder.last() {
                        if ['a', 'b', 'c', 'd', 'e', 'f'].contains(&last_pushed) {
                            builder.extend(['_', 's', 'h', 'a', 'r', 'p']);
                        } else if let Some(&next_char) = input.get(i + 1) {
                            if next_char.is_numeric() {
                                builder.extend(['n', 'u', 'm', 'b', 'e', 'r', '_']);
                            } else {
                                builder.extend(['h', 'a', 's', 'h']);
                            }
                        } else {
                            builder.extend(['n', 'u', 'm', 'b', 'e', 'r']);
                        }
                    } else {
                        builder.extend(['n', 'u', 'm', 'b', 'e', 'r', '_']);
                    }
                }
                '@' => builder.extend(['a', 't']),
                '&' => builder.extend(['a', 'n', 'd']),
                '$' => {
                    let next_char = input.get(i + 1);
                    if next_char.is_none() || !&next_char.unwrap().is_numeric() {
                        builder.extend(['d', 'o', 'l', 'l', 'a', 'r'])
                    }
                }
                '=' => builder.extend(['e', 'q', 'u', 'a', 'l', 's']),
                '%' => {
                    if let Some(&last_pushed) = builder.last() {
                        if last_pushed != '_' || last_pushed != '-' {
                            builder.push('_');
                        }
                    }
                    builder.extend(['p', 'e', 'r', 'c', 'e', 'n', 't']);
                }
                _ => {}
            }
        }

        if builder.is_empty() {
            self.all_symbols()
            // "no_title".into()
        } else {
            builder
                .iter()
                .cloned()
                .collect::<String>()
                .trim_end_matches("-")
                .to_string()
        }
    }

    fn all_symbols(&self) -> String {
        let words: Vec<&str> = self.split_whitespace().collect();

        words
            .iter()
            .map(|w| {
                if w.len() == 1 {
                    sym_to_word(w)
                } else {
                    w.chars()
                        .map(|c| sym_to_word(&c.to_string()))
                        .collect::<Vec<String>>()
                        .join("-")
                }
            })
            .collect::<Vec<String>>()
            .join("_")
    }
}

fn sym_to_word(symbol: &str) -> String {
    let ret = match symbol {
        "!" => "bang",
        "#" => "hash",
        "$" => "dollar",
        "£" => "pound",
        "%" => "percent",
        "'" => "tick",
        "*" => "star",
        "-" => "dash",
        "." => "dot",
        "/" => "slash",
        ":" => "colon",
        ";" => "semicolon",
        "@" => "at",
        "[" | "(" | ")" | "]" => "bracket",
        "\\" => "backslash",
        "_" => "underscore",
        "`" => "backtick",
        "|" => "pipe",
        _ => "",
    };

    ret.to_string()
}

impl ToFilenameChunk for &str {
    fn to_filename_chunk(&self) -> String {
        self.to_string().to_filename_chunk()
    }

    fn all_symbols(&self) -> String {
        self.to_string().all_symbols()
    }
}

impl ReplaceLast for String {
    fn replace_last(&self, from: &str, to: &str) -> String {
        if let Some(idx) = self.rfind(from) {
            let (start, end) = self.split_at(idx);
            format!("{}{}{}", start, to, &end[from.len()..])
        } else {
            self.to_string()
        }
    }
}

impl ReplaceLast for &str {
    fn replace_last(&self, from: &str, to: &str) -> String {
        self.to_string().replace_last(from, to)
    }
}

impl Compacted for String {
    fn compacted(&self) -> String {
        let mut ret = String::new();

        for c in self.to_lowercase().chars() {
            if c.is_alphanumeric() {
                ret.push(c);
            }
        }

        ret
    }
}

impl Compacted for &str {
    fn compacted(&self) -> String {
        self.to_string().compacted()
    }
}

impl Capitalize for String {
    fn capitalize(&self) -> String {
        let lc = self.to_lowercase();
        let mut chars = lc.chars();
        if let Some(first) = chars.next() {
            first.to_uppercase().collect::<String>() + chars.as_str()
        } else {
            String::new()
        }
    }
}

impl Capitalize for &str {
    fn capitalize(&self) -> String {
        self.to_string().capitalize()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_filename_chunk() {
        let tests = [
            ("", ""),
            ("basic", "basic"),
            ("Füxa", "fuxa"),
            ("R.E.M.", "r-e-m"),
            ("St. Christopher", "st_christopher"),
            ("20 Cases Suggestive of...", "20_cases_suggestive_of"),
            ("V/Vm", "v--vm"),
            ("Say \"Yes!\"", "say_yes"),
            ("...Baby One More Time", "baby_one_more_time"),
            ("simple-String", "simple-string"),
            ("Simple String", "simple_string"),
            ("Incident @ 23rd", "incident_at_23rd"),
            ("+2", "plus2"),
            ("Stripped String  ", "stripped_string"),
            ("I Feel Love (12\" mix)", "i_feel_love--12_inch_mix"),
            (
                "a long, complicated string-type-thing.",
                "a_long_complicated_string-type-thing",
            ),
            ("!|~~c*o*n*t*e*n*t~~,:", "c-o-n-t-e-n-t"),
            (
                "Looking for Love (in the Hall of Mirrors)",
                "looking_for_love--in_the_hall_of_mirrors",
            ),
            (
                "(You Gotta) Fight for Your Right (to Party!)",
                "you_gotta--fight_for_your_right--to_party",
            ),
            ("this is (almost) too easy", "this_is--almost--too_easy"),
            ("I'm almost sure you're not...", "im_almost_sure_youre_not"),
            ("Hello, Dad... I'm in Jail", "hello_dad_im_in_jail"),
            ("Btwn You + Me", "btwn_you_and_me"),
            ("Approach / Descend", "approach--descend"),
            (
                "Wu-Tang: 7th Chamber - Part II / Conclusion",
                "wu-tang--7th_chamber--part_ii--conclusion",
            ),
            ("Jon Brooks and Sean O'Hagan", "jon_brooks_and_sean_o-hagan"),
            ("St. Elmo's Fire (Red Corona)", "st_elmos_fire--red_corona"),
            (
                "Outrun (Negative Space) (Vinyl Edit)",
                "outrun--negative_space--vinyl_edit",
            ),
            ("L'age D'or", "l-age_d-or"),
            ("The Man don't Give a F**k", "the_man_dont_give_a_f--k"),
            (
                "Never Nothing (It's Alright [It's OK])",
                "never_nothing--its_alright--its_ok",
            ),
            (
                "Little Requiems (Voiceless Mix) - Part 3",
                "little_requiems--voiceless_mix--part_3",
            ),
            ("Missile ++", "missile_plus_plus"),
            ("Barney (...And Me)", "barney--and_me"),
            ("1000%", "1000_percent"),
            ("Who (Will Take My Place)?", "who--will_take_my_place"),
            ("010 +- 4.40", "010_plus-minus_4-40"),
            ("£24.99 from Argos", "24-99_from_argos"),
            ("We Failed (...To Break Up!)", "we_failed--to_break_up"),
            ("Like 24 (6+1=3)", "like_24--6plus1equals3"),
            ("Juneau/Projects/", "juneau--projects"),
            (
                "Arbol and ...Unexplained Transmissions",
                "arbol_and_unexplained_transmissions",
            ),
            (
                "Carrying the Fire: Prelude in F# Minor and Launch",
                "carrying_the_fire--prelude_in_f_sharp_minor_and_launch",
            ),
            ("$99.99", "99-99"),
            ("The Train #2", "the_train_number_2"),
            ("*Break*", "break"),
            ("180db_", "180db"),
            ("#302", "number_302"),
            ("Latin #", "latin_number"),
            ("(...)", "bracket-dot-dot-dot-bracket"),
            ("# * ! @ !", "hash_star_bang_at_bang"),
            ("1", "1"),
            ("x", "x"),
            (".", "dot"),
            (
                "_______",
                "underscore-underscore-underscore-underscore-underscore-underscore-underscore",
            ),
        ];

        for (input, output) in tests {
            assert_eq!(output.to_string(), input.to_filename_chunk());
        }
    }

    #[test]
    fn test_replace_last() {
        assert_eq!("filename.mp3", "filename.flac".replace_last("flac", "mp3"));
        assert_eq!("flacname.mp3", "flacname.flac".replace_last("flac", "mp3"));
        assert_eq!("me_me_me_you", "me_me_me_me".replace_last("me", "you"));
    }

    #[test]
    fn test_compacted() {
        assert_eq!("theb52s", "The B52s".compacted());
        assert_eq!("theb52s", "The B52's".compacted());
        assert_eq!("theb52s", "The B-52's".compacted());
    }

    #[test]
    fn test_capitalize() {
        assert_eq!("Merp", "merp".capitalize());
        assert_eq!("Merp", "MERP".capitalize());
    }
}
