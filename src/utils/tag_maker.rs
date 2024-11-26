fn title_from(string: &str) -> String {
    let mut in_brackets = false;
    let str: Vec<_> = string.split('_').collect();

    let bits: Vec<&str> = str
        .iter()
        .enumerate()
        .map(|(i, s)| handle_string(s, i, str.len()))
        .collect();

    join_up(&bits, in_brackets)
}

fn handle_string(string: &str, index: usize, count: usize) -> &str {
    let chars: Vec<_> = string.chars().collect();

    if chars.len() >= 3 && chars[0].is_alphabetic() && chars[1] == '-' && chars[2].is_alphabetic() {
        handle_initials(string)
    } else if string.contains("--") {
        handle_long_dash(string)
    } else if string.contains("-") {
        handle_short_dash(string)
    } else {
        smart_capitalize(string, index, count)
    }
}

fn handle_initials(string: &str) -> &str {
    string
}

fn handle_long_dash(string: &str) -> &str {
    string
}

fn handle_short_dash(string: &str) -> &str {
    string
}

fn smart_capitalize(string: &str, index: usize, count: usize) -> &str {
    string
}

fn join_up(words: &Vec<&str>, in_brackets: bool) -> String {
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
        assert_eq!("Blue Bell Knoll", title_from("blue_bell_knoll"));
    }

    //   fn test_title_the() {
    //     assert_eq!("The Boy with the Arab Strap",;
    //                  title_from("the_boy_with_the_arab_strap"))
    //   }

    //   fn test_title_caps() {
    //     assert_eq!("Enemies EP", title_from("enemies_ep"));
    //   }

    //   fn test_title_last_word_preposition() {
    //     assert_eq!("I Want to Wake Up", title_from("i_want_to_wake_up"));
    //   }

    //   fn test_title_contraction() {
    //     assert_eq!("Don"t Go", title_from("dont_go"));
    //     assert_eq!("You Can"t Hold What You Haven"t Got in Your Hand",;
    //                  title_from("you_cant_hold_what_you_havent_got_in_your_hand"))
    //   }

    //   fn test_title_inches() {
    //     assert_eq!("I Feel Love (12" Mix)", title_from("i_feel_love--12inch_mix"));
    //     assert_eq!("Fugitive (7" Mix)", title_from("fugitive--7inch_mix"));
    //   }

    //   fn test_title_hyphen() {
    //     assert_eq!("When-Never", title_from("when-never"));
    //     assert_eq!("Who-What and Maybe-Not", title_from("who-what_and_maybe-not"));
    //     assert_eq!("Tick-Tock-Tick-Tock", title_from("tick-tock-tick-tock"));
    //   }

    //   fn test_title_brackets_in_middle() {
    //     assert_eq!("This Is (Almost) Too Easy",;
    //                  title_from("this_is--almost--too_easy"))
    //     assert_eq!("Can We Make It (Just a Little Bit) Harder",;
    //                  title_from("can_we_make_it--just_a_little_bit--harder"))
    //     assert_eq!("Title }ing In (Bracketed Words)",;
    //                  title_from("title_}ing_in--bracketed_words"))
    //     assert_eq!("Two (Lots) Of Brackets (Is Tricky)",;
    //                  title_from("two--lots--of_brackets--is_tricky"))
    //     assert_eq!("Variations 3 (Canon on the Unison)",;
    //                  title_from("variations_3--canon_on_the_unison"))
    //   }

    //   fn test_title_brackets_at_}() {
    //     assert_eq!("Suburbia (The Full Horror)",;
    //                  title_from("suburbia--the_full_horror"))
    //     assert_eq!("Om Mani Padme Hum 3 (Piano Version)",;
    //                  title_from("om_mani_padme_hum_3--piano_version"))
    //     assert_eq!("Drumming (Part III)", title_from("drumming--part_iii"));
    //   }

    //   fn test_title_initials() {
    //     assert_eq!("C.R.E.E.P.", title_from("c-r-e-e-p"));
    //     assert_eq!("The N.W.R.A.", title_from("the_n-w-r-a"));
    //     assert_eq!("W.M.C. Blob 59", title_from("w-m-c_blob_59"));
    //   }

    //   fn test_initials_in_brackets() {
    //     assert_eq!("The (I.N.I.T.I.A.L.S.) In Brackets",;
    //                  title_from("the--i-n-i-t-i-a-l-s--in_brackets"))
    //   }

    //   fn test_artist() {
    //     assert_eq!("Stereolab", artist_from("stereolab"));
    //     assert_eq!("Royal Trux", artist_from("royal_trux"));
    //     assert_eq!("Jeffrey Lewis and The Bolts",;
    //                  artist_from("jeffrey_lewis_and_the_bolts"))
    //     assert_eq!("Someone featuring Someone Else",;
    //                  artist_from("someone_ft_someone_else"))
    //     assert_eq!("R.E.M.", artist_from("r-e-m"));
    //     assert_eq!("M.J. Hibbett", artist_from("m-j_hibbett"));
    //     assert_eq!("ABBA", artist_from("abba"));
    //     assert_eq!("Add N to (X)", artist_from("add_n_to_x"));
    //   }

    //   fn test_album() {
    //     assert_eq!("Spiderland", album_from("spiderland"));
    //     assert_eq!("Mars Audiac Quintet", album_from("mars_audiac_quintet"));
    //     assert_eq!("The Decline and Fall of Heavenly",;
    //                  album_from("the_decline_and_fall_of_heavenly"))
    //   }

    //   fn test_genre() {
    //     assert_eq!("Noise", genre_from("noise"));
    //     assert_eq!("Noise", genre_from("Noise "));
    //     assert_eq!("Hip-Hop", genre_from("Hip-Hop"));
    //     assert_eq!("Rock and Roll", genre_from("rock AND roll"));
    //   }

    //   fn test_t_num() {
    //     assert_eq!("1", t_num_from("01"));
    //     assert_eq!("39", t_num_from("39"));
    //   }
}
