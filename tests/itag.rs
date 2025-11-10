#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_itag_command_renumber() {
        let file_name = "01.original_artist.original_title.flac";

        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("t_num")
            .arg(&file_under_test)
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

        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("t_num")
            .arg(&new_file_under_test)
            .write_stdin("5\n")
            .assert()
            .success()
            .stdout("05.original_artist.original_title.flac [t_num]> ");
    }

    #[test]
    #[ignore]
    fn test_itag_command_change_artist() {
        let file_name = "01.original_artist.original_title.flac";

        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("artist")
            .arg(&file_under_test)
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

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("artist")
            .arg(&new_file_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "New Artist : {new_file_under_test}",
            )));

        // Nothing should happen this time, but we'll still get the prompt

        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("artist")
            .arg(&new_file_under_test)
            .write_stdin("New Artist")
            .assert()
            .success()
            .stdout("01.new_artist.original_title.flac [artist]> ");
    }

    #[test]
    #[ignore]
    fn test_itag_command_change_year() {
        let file_name = "02.original_artist.original_title.mp3";

        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/itag"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("year")
            .arg(&file_under_test)
            .write_stdin("2024")
            .assert()
            .success()
            .stdout("02.original_artist.original_title.mp3 [year]> ");

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("-s")
            .arg("year")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("2024\n");

        // Nothing should happen this time, but we'll still get the prompt

        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("year")
            .arg(&file_under_test)
            .write_stdin("2024")
            .assert()
            .success()
            .stdout("02.original_artist.original_title.mp3 [year]> ");
    }

    #[test]
    #[ignore]
    fn test_itag_command_missing_file() {
        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("artist")
            .arg("/no/such/file.flac")
            .write_stdin("1")
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_itag_bad_input() {
        let file = fixture_as_string("commands/itag/01.original_artist.original_title.flac");

        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("t_num")
            .arg(file)
            .write_stdin("merp")
            .assert()
            .failure()
            .stdout("01.original_artist.original_title.flac [t_num]> ")
            .stderr("ERROR: 'merp' is not a valid t_num value\n");
    }

    #[test]
    #[ignore]
    fn test_itag_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("itag")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        cargo_bin_cmd!("aur")
            .arg("itag")
            .arg("title")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
