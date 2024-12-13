mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture_as_string;

    #[test]
    #[ignore]
    fn test_lintdir_command() {
        assert_cli::Assert::main_binary()
            .with_args(&[
                "lintdir",
                &fixture_as_string("commands/lintdir/flac/tester.perfect"),
                &fixture_as_string("commands/lintdir/mp3/tester.perfect"),
                &fixture_as_string("commands/lintdir/mp3/artist--band.split_single"),
                &fixture_as_string("commands/lintdir/mp3/various.compilation"),
                &fixture_as_string("commands/lintdir/mp3/tester.perfect--featuring"),
                &fixture_as_string("commands/lintdir/mp3/tester.bonus_disc"),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "lintdir",
                &fixture_as_string("commands/lintdir/flac/tester.artwork_too_small"),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("Cover art is too small")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "lintdir",
                &fixture_as_string("commands/lintdir/flac/tester.artwork_too_big"),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("Cover art is too big")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "lintdir",
                &fixture_as_string("commands/lintdir/flac/tester.artwork_not_square"),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("Cover art is not square")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "lintdir",
                &fixture_as_string("commands/lintdir/mp3/tester.unwanted_art"),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains(
                format!(
                    "Unexpected file(s): {}/front.jpg",
                    fixture_as_string("commands/lintdir/mp3/tester.unwanted_art"),
                )
                .as_str(),
            )
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "lintdir",
                &fixture_as_string("commands/lintdir/mp3/tester.mixed_genre_year_album"),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("Inconsistent tags: album, genre, year")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_lintdir_incorrect_usage() {
        common::missing_file_args_test("lintdir");
    }
}
