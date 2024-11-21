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
}
