#[cfg(test)]
mod test {
    use aur::test_utils::spec_helper::{config_file_as_string, fixture_as_string, sample_output};

    #[test]
    #[ignore]
    fn test_wantflac_command_valid_tree() {
        let dir_under_test = fixture_as_string("commands/wantflac");

        assert_cli::Assert::main_binary()
            .with_args(&[
                "--config",
                config_file_as_string().as_str(),
                "wantflac",
                "--root",
                dir_under_test.as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is(sample_output("commands/wantflac/wantflac.txt").as_str())
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_wantflac_command_invalid_tree() {
        assert_cli::Assert::main_binary()
            .with_args(&["wantflac", "--root", "/tmp"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: did not find /tmp/mp3")
            .unwrap();
    }
}
