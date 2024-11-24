mod common;

#[cfg(test)]
mod test {
    use super::common;
    use aur::test_utils::spec_helper::fixture_as_string;

    #[test]
    #[ignore]
    fn test_namecheck_command_valid_tree() {
        // this test is rubbish. make it better
        let dir_under_test = fixture_as_string("commands/namecheck");

        assert_cli::Assert::main_binary()
            .with_args(&["namecheck", dir_under_test.as_str()])
            .succeeds()
            .and()
            .stdout()
            .contains("Artist")
            .and()
            .stdout()
            .contains("The B-52's")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_namecheck_command_invalid_tree() {
        assert_cli::Assert::main_binary()
            .with_args(&["namecheck", "/no/such/dir"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: No files found")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_namecheck_incorrect_usage() {
        common::missing_file_args_test("namecheck");
    }
}
