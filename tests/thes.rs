#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_thes_command() {
        let file_name = "test.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file_name]).unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&[
                "thes",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("artist -> The Test Artist")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "thes",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }
}
