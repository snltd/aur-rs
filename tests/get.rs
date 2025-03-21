#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use aur::test_utils::spec_helper::fixture_as_string;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_get_command_valid_property() {
        let file_under_test = fixture_as_string("commands/tags/01.test_artist.test_track.flac");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "title", &file_under_test])
            .assert()
            .success()
            .stdout(predicate::str::ends_with(format!(
                "Test Track : {}\n",
                file_under_test
            )));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "bitrate", &file_under_test])
            .assert()
            .success()
            .stdout(predicate::str::ends_with(format!(
                "16-bit/44100Hz : {}\n",
                file_under_test
            )));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "bitrate", "-s", &file_under_test])
            .assert()
            .success()
            .stdout("16-bit/44100Hz\n");
    }

    #[test]
    #[ignore]
    fn test_get_command_invalid_property() {
        let fixture = fixture_as_string("commands/tags/01.test_artist.test_track.flac");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "whatever", &fixture])
            .assert()
            .failure()
            .stderr("ERROR: Unknown tag: whatever\n");
    }

    #[test]
    #[ignore]
    fn test_get_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "title", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_get_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("get")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "title"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
