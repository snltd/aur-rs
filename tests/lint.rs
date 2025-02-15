mod common;

#[cfg(test)]
mod test {
    use super::common;
    use aur::test_utils::spec_helper::fixture_as_string;

    #[test]
    #[ignore]
    fn test_lint_command_missing_tags() {
        assert_cli::Assert::main_binary()
            .with_args(&[
                "lint",
                &fixture_as_string("commands/lint/00.tester.missing_genre_track_no_year.flac"),
            ])
            .stdout()
            .contains("Invalid track number tag: 0")
            .and()
            .stdout()
            .contains("Invalid year tag: 0")
            .and()
            .stdout()
            .contains("Invalid genre tag: ")
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
    fn test_lint_command_respects_config() {
        let file_to_test = fixture_as_string("commands/lint/09.tester.bad_title_allowed.mp3");
        assert_cli::Assert::main_binary()
            .with_args(&["lint", &file_to_test])
            .stdout()
            .contains("Invalid title tag: BAD title allo,Wed")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "lint",
                "--config",
                &fixture_as_string("config/test.toml"),
                &file_to_test,
            ])
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
