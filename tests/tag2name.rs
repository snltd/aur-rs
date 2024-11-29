mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_tag2name_command() {
        let file_name = "test.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file_name]).unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&["tag2name", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("test.flac -> 06.test_artist.test_title.flac")
            .unwrap();

        let renamed_file = tmp.path().join("06.test_artist.test_title.flac");

        assert_cli::Assert::main_binary()
            .with_args(&["tag2name", &renamed_file.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tag2name_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["tag2name", "/no/such/file.flac"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tag2name_incorrect_usage() {
        common::missing_file_args_test("tag2name");
    }
}
