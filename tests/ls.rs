#[cfg(test)]
mod test {
    use aur::test_utils::spec_helper::{fixture_as_string, sample_output};

    #[test]
    #[ignore]
    fn test_ls_command() {
        assert_cli::Assert::main_binary()
            .with_args(&["ls", fixture_as_string("commands/ls").as_str()])
            .succeeds()
            .and()
            .stdout()
            .is(sample_output("commands/ls/ls.txt").as_str())
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["ls"])
            .current_dir(fixture_as_string("commands/ls").as_str())
            .succeeds()
            .and()
            .stdout()
            .is(sample_output("commands/ls/ls.txt").as_str())
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_ls_command_no_media() {
        assert_cli::Assert::main_binary()
            .with_args(&["ls", "/"])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .and()
            .stderr()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_ls_command_no_dir() {
        assert_cli::Assert::main_binary()
            .with_args(&["ls", "/no/such/directory"])
            .fails()
            .and()
            .stdout()
            .is("")
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }
}
