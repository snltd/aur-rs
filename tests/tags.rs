#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string, sample_output};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_tags_command() {
        let expected_output = format!(
            "{}\n{}",
            fixture("commands/tags/01.test_artist.test_track.flac"),
            sample_output("commands/tags/01.test_artist.test_track.flac.txt")
        );

        Command::cargo_bin("aur")
            .unwrap()
            .arg("tags")
            .arg(fixture_as_string(
                "commands/tags/01.test_artist.test_track.flac",
            ))
            .assert()
            .success()
            .stdout(expected_output);
    }

    #[test]
    #[ignore]
    fn test_tags_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["tags", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_tags_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("tags")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
