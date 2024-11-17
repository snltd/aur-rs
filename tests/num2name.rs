#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    fn test_cli() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&[
                "num2name",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("01.test_artist.test_title.flac -> 02.test_artist.test_title.flac")
            .unwrap();

        let renamed_file = tmp.path().join("02.test_artist.test_title.flac");

        assert_cli::Assert::main_binary()
            .with_args(&[
                "num2name",
                renamed_file.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }
}
