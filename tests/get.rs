#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture_as_string;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_get_command_valid_property() {
        let file_under_test = fixture_as_string("commands/tags/01.test_artist.test_track.flac");

        cargo_bin_cmd!("aur")
            .args(["get", "title", &file_under_test])
            .assert()
            .success()
            .stdout(predicate::str::ends_with(format!(
                "Test Track : {file_under_test}\n",
            )));

        cargo_bin_cmd!("aur")
            .args(["get", "bitrate", &file_under_test])
            .assert()
            .success()
            .stdout(predicate::str::ends_with(format!(
                "16-bit/44100Hz : {file_under_test}\n",
            )));

        cargo_bin_cmd!("aur")
            .args(["get", "bitrate", "-s", &file_under_test])
            .assert()
            .success()
            .stdout("16-bit/44100Hz\n");
    }

    #[test]
    #[ignore]
    fn test_get_command_invalid_property() {
        let fixture = fixture_as_string("commands/tags/01.test_artist.test_track.flac");

        cargo_bin_cmd!("aur")
            .args(["get", "whatever", &fixture])
            .assert()
            .failure()
            .stderr("ERROR: Unknown tag: whatever\n");
    }

    #[test]
    #[ignore]
    fn test_get_command_missing_file() {
        cargo_bin_cmd!("aur")
            .args(["get", "title", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_get_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("get")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        cargo_bin_cmd!("aur")
            .args(["get", "title"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
