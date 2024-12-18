mod common;

#[cfg(test)]
mod test {
    use super::common;
    use aur::test_utils::spec_helper::fixture_as_string;

    #[test]
    #[ignore]
    fn test_verify_command_valid_tree() {
        let dir_under_test = fixture_as_string("commands/verify");

        assert_cli::Assert::main_binary()
            .with_args(&["verify", "-r", &dir_under_test])
            .succeeds()
            .and()
            .stdout()
            .contains("04.tester.truncated.mp3")
            .and()
            .stdout()
            .contains("02.tester.truncated.flac")
            .and()
            .stdout()
            .contains("06.tester.junk.mp3")
            .and()
            .stdout()
            .contains("05.tester.junk.flac")
            .and()
            .stdout()
            .doesnt_contain("OK")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["verify", "-r", "-v", &dir_under_test])
            .succeeds()
            .and()
            .stdout()
            .contains("04.tester.truncated.mp3")
            .and()
            .stdout()
            .contains("02.tester.truncated.flac")
            .and()
            .stdout()
            .contains("06.tester.junk.mp3")
            .and()
            .stdout()
            .contains("05.tester.junk.flac")
            .and()
            .stdout()
            .contains("OK")
            .and()
            .stdout()
            .contains("01.tester.valid.flac")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_inumber_incorrect_usage() {
        common::missing_file_args_test("verify");
    }
}
