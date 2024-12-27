mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_itag_command_change_artist() {
        let file_name = "01.test_artist.untagged_song.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&["itag", "artist", &file_under_test.to_string_lossy()])
            .stdin("New Artist")
            .succeeds()
            .and()
            .stdout()
            .contains("01.test_artist.untagged_song.flac > ")
            .and()
            .stdout()
            .contains("artist -> New Artist")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "artist", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("New Artist")
            .unwrap();

        // Nothing should happen this time, but we'll still get the prompt

        assert_cli::Assert::main_binary()
            .with_args(&["itag", "artist", &file_under_test.to_string_lossy()])
            .stdin("New Artist")
            .succeeds()
            .and()
            .stdout()
            .is("01.test_artist.untagged_song.flac > ")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_itag_command_change_year() {
        let file_name = "01.test_artist.untagged_song.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&["itag", "year", &file_under_test.to_string_lossy()])
            .stdin("2024")
            .succeeds()
            .and()
            .stdout()
            .contains("01.test_artist.untagged_song.mp3 > ")
            .and()
            .stdout()
            .contains("year -> 2024")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "year", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("2024 : ")
            .unwrap();

        // Nothing should happen this time, but we'll still get the prompt

        assert_cli::Assert::main_binary()
            .with_args(&["itag", "year", &file_under_test.to_string_lossy()])
            .stdin("2024")
            .succeeds()
            .and()
            .stdout()
            .is("01.test_artist.untagged_song.mp3 > ")
            .unwrap();
    }

    // #[test]
    // #[ignore]
    // fn test_itag_command_missing_file() {
    //     assert_cli::Assert::main_binary()
    //         .with_args(&["itag", "/no/such/file.flac"])
    //         .stdin("1")
    //         .fails()
    //         .and()
    //         .stderr()
    //         .is("ERROR: (I/O) : No such file or directory (os error 2)")
    //         .unwrap();
    // }

    // #[test]
    // #[ignore]
    // fn test_itag_bad_input() {
    //     assert_cli::Assert::main_binary()
    //         .with_args(&["itag", "/no/such/file.flac"])
    //         .stdin("merp")
    //         .fails()
    //         .and()
    //         .stderr()
    //         .is("ERROR: (Parsing): invalid digit found in string")
    //         .unwrap();
    // }

    // #[test]
    // #[ignore]
    // fn test_itag_incorrect_usage() {
    //     common::missing_file_args_test("itag");
    // }
}
