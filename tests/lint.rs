mod common;

#[cfg(test)]
mod test {
    use super::common;
    use aur::test_utils::spec_helper::{fixture_as_string, sample_output};

    #[test]
    #[ignore]
    fn test_lint_command_missing_tags() {
        assert_cli::Assert::main_binary()
            .with_args(&[
                "lint",
                &fixture_as_string("commands/lint/00.tester.missing_genre_track_no_year.flac"),
            ])
            .stdout()
            .contains("Bad: invalid name")
            .and()
            .stdout()
            .contains("Bad: invalid t_num: 0")
            .and()
            .stdout()
            .contains("Bad: invalid year: 0")
            .and()
            .stdout()
            .contains("Bad: invalid genre: ")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_lint_command_fine() {
        assert_cli::Assert::main_binary()
            .with_args(&[
                "lint",
                &fixture_as_string("commands/lint/01.tester.lints_fine.flac"),
                &fixture_as_string("commands/lint/02.tester.lints_fine.mp3"),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_lint_incorrect_usage() {
        common::missing_file_args_test("lint");
    }
}
