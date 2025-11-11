#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::{fixture_as_string, sample_output};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_info_command_valid_file() {
        cargo_bin_cmd!("aur")
            .arg("info")
            .arg(fixture_as_string(
                "commands/tags/01.test_artist.test_track.flac",
            ))
            .assert()
            .success()
            .stdout(sample_output(
                "commands/info/01.test_artist.test_track.flac.txt",
            ));
    }

    #[test]
    #[ignore]
    fn test_info_command_missing_file() {
        cargo_bin_cmd!("aur")
            .args(["info", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_info_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("info")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
