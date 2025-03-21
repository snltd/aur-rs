#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_itag_command_renumber() {
        let file_name = "01.original_artist.original_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "t_num", &file_under_test.to_string_lossy()])
            .write_stdin("5\n")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "01.original_artist.original_title.flac [t_num]> ",
            ))
            .stdout(predicate::str::contains(
                "01.original_artist.original_title.flac -> 05.original_artist.original_title.flac",
            ));

        // Nothing should happen this time, but we'll still get the prompt

        let new_file_under_test = tmp.path().join("05.original_artist.original_title.flac");
        assert!(new_file_under_test.exists());

        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "t_num", &new_file_under_test.to_string_lossy()])
            .write_stdin("5\n")
            .assert()
            .success()
            .stdout("05.original_artist.original_title.flac [t_num]> ");
    }

    #[test]
    #[ignore]
    fn test_itag_command_change_artist() {
        let file_name = "01.original_artist.original_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "artist", &file_under_test.to_string_lossy()])
            .write_stdin("New Artist")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "01.original_artist.original_title.flac [artist]> ",
            ))
            .stdout(predicate::str::contains(
                "01.original_artist.original_title.flac -> 01.new_artist.original_title.flac\n",
            ));

        let new_file_under_test = tmp.path().join("01.new_artist.original_title.flac");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "artist", &new_file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "New Artist : {}",
                new_file_under_test.to_string_lossy()
            )));

        // Nothing should happen this time, but we'll still get the prompt

        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "artist", &new_file_under_test.to_string_lossy()])
            .write_stdin("New Artist")
            .assert()
            .success()
            .stdout("01.new_artist.original_title.flac [artist]> ");
    }

    #[test]
    #[ignore]
    fn test_itag_command_change_year() {
        let file_name = "02.original_artist.original_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "year", &file_under_test.to_string_lossy()])
            .write_stdin("2024")
            .assert()
            .success()
            .stdout("02.original_artist.original_title.mp3 [year]> ");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "-s", "year", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("2024\n");

        // Nothing should happen this time, but we'll still get the prompt

        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "year", &file_under_test.to_string_lossy()])
            .write_stdin("2024")
            .assert()
            .success()
            .stdout("02.original_artist.original_title.mp3 [year]> ");
    }

    #[test]
    #[ignore]
    fn test_itag_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "artist", "/no/such/file.flac"])
            .write_stdin("1")
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_itag_bad_input() {
        let file_under_test =
            fixture_as_string("commands/itag/01.original_artist.original_title.flac");
        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "t_num", &file_under_test])
            .write_stdin("merp")
            .assert()
            .success()
            .stdout("01.original_artist.original_title.flac [t_num]> ")
            .stderr("ERROR: 'merp' is not a valid t_num value\n");
    }

    #[test]
    #[ignore]
    fn test_itag_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("itag")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["itag", "title"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
