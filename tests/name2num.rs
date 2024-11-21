#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_name2num_command() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&[
                "name2num",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("t_num -> 1")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "name2num",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }
}
