mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_set_command() {
        let file_name = "test.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file_name]).unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&[
                "set",
                "title",
                "New Title",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .stdout()
            .is("title -> New Title")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "set",
                "artist",
                "Test Artist",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_set_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["set", "title", "new title", "/no/such/file.flac"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_set_command_bad_input() {
        let file_name = "test.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("info"), &[file_name]).unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&[
                "set",
                "whatever",
                "new title",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .fails()
            .and()
            .stderr()
            .is("ERROR: Unknown tag name")
            .unwrap();

        // This should probably fail. Verify tags?
        // assert_cli::Assert::main_binary()
        //     .with_args(&[
        //         "set",
        //         "t_num",
        //         "five",
        //         file_under_test.to_string_lossy().to_string().as_str(),
        //     ])
        //     .fails()
        //     .and()
        //     .stderr()
        //     .is("ERROR: Unknown tag name")
        //     .unwrap();
    }

    #[test]
    #[ignore]
    fn test_set_incorrect_usage() {
        common::missing_file_args_test("set");
    }
}
