mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_tagsub_command_change() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();
        assert_cli::Assert::main_binary()
            .with_args(&["--verbose", "tagsub", "artist", "Test", "Tested", &file_str])
            .stdout()
            .contains("06.test_artist.test_title.mp3: Test Artist -> Tested Artist")
            .and()
            .stdout()
            .contains("artist -> Tested Artist")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "artist", &file_str])
            .stdout()
            .contains("Tested Artist : ")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_noop() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();
        assert_cli::Assert::main_binary()
            .with_args(&["--noop", "tagsub", "artist", "Test", "Tested", &file_str])
            .stdout()
            .contains("06.test_artist.test_title.mp3: Test Artist -> Tested Artist")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "artist", &file_str])
            .stdout()
            .contains("Test Artist : ")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["tagsub", "title", "find", "replace", "/no/such/file.flac"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_bad_input() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();
        assert_cli::Assert::main_binary()
            .with_args(&["tagsub", "whatever", "find", "replace", &file_str])
            .fails()
            .and()
            .stderr()
            .is("ERROR: Unknown tag: whatever")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tagsub_incorrect_usage() {
        common::missing_file_args_test("tagsub");
    }
}
