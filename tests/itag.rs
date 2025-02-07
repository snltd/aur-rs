mod common;

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};

    #[test]
    #[ignore]
    fn test_itag_command_renumber() {
        let file_name = "01.original_artist.original_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&["itag", "t_num", &file_under_test.to_string_lossy()])
            .stdin("5")
            .succeeds()
            .and()
            .stdout()
            .contains("01.original_artist.original_title.flac [t_num]> ")
            .and()
            .stdout()
            .contains(
                "01.original_artist.original_title.flac -> 05.original_artist.original_title.flac",
            )
            .unwrap();

        // Nothing should happen this time, but we'll still get the prompt

        let new_file_under_test = tmp.path().join("05.original_artist.original_title.flac");

        assert!(new_file_under_test.exists());

        assert_cli::Assert::main_binary()
            .with_args(&["itag", "t_num", &new_file_under_test.to_string_lossy()])
            .stdin("5")
            .succeeds()
            .and()
            .stdout()
            .is("05.original_artist.original_title.flac [t_num]> ")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_itag_command_change_artist() {
        let file_name = "01.original_artist.original_title.flac";
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
            .contains("01.original_artist.original_title.flac [artist]> ")
            .and()
            .stdout()
            .contains("01.original_artist.original_title.flac -> 01.new_artist.original_title.flac")
            .unwrap();

        let new_file_under_test = tmp.path().join("01.new_artist.original_title.flac");

        assert_cli::Assert::main_binary()
            .with_args(&["get", "artist", &new_file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("New Artist")
            .unwrap();

        // Nothing should happen this time, but we'll still get the prompt

        assert_cli::Assert::main_binary()
            .with_args(&["itag", "artist", &new_file_under_test.to_string_lossy()])
            .stdin("New Artist")
            .succeeds()
            .and()
            .stdout()
            .is("01.new_artist.original_title.flac [artist]> ")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_itag_command_change_year() {
        let file_name = "02.original_artist.original_title.mp3";
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
            .is("02.original_artist.original_title.mp3 [year]> ")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "-s", "year", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("2024")
            .unwrap();

        // Nothing should happen this time, but we'll still get the prompt

        assert_cli::Assert::main_binary()
            .with_args(&["itag", "year", &file_under_test.to_string_lossy()])
            .stdin("2024")
            .succeeds()
            .and()
            .stdout()
            .is("02.original_artist.original_title.mp3 [year]> ")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_itag_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["itag", "artist", "/no/such/file.flac"])
            .stdin("1")
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_itag_bad_input() {
        let file_under_test =
            fixture_as_string("commands/itag/01.original_artist.original_title.flac");
        assert_cli::Assert::main_binary()
            .with_args(&["itag", "t_num", &file_under_test])
            .stdin("merp")
            .fails()
            .and()
            .stderr()
            .is("ERROR: (Parsing): invalid digit found in string")
            .unwrap();
    }

    // #[test]
    // #[ignore]
    // fn test_itag_incorrect_usage() {
    //     common::missing_file_args_test("itag");
    // }
}
