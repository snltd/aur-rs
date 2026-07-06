#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use predicates::prelude::*;
    use snltest::{fixture, load_fixture};

    #[test]
    #[ignore]
    fn test_info_command_valid_file() {
        cargo_bin_cmd!("aur")
            .arg("info")
            .arg(fixture!("commands/tags/01.test_artist.test_track.flac"))
            .assert()
            .success()
            .stdout(load_fixture!(
                "outputs/commands/info/01.test_artist.test_track.flac.txt"
            ));
    }

    #[test]
    #[ignore]
    fn test_info_command_missing_file() {
        cargo_bin_cmd!("aur")
            .args(["info", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr(
                "Error getting metadata for /no/such/file.flac: No such file or directory (os error 2)\n");
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
