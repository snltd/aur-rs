#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_dupes_command_valid_tree() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/dupes"), &["flac/**/*"])
            .unwrap();
        let file_under_test = tmp.path().join("flac");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["dupes", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "flac/tracks/fall.free_ranger.flac",
            ))
            .stdout(predicate::str::contains("flac/tracks/slint.don_aman.flac"));
    }

    #[test]
    #[ignore]
    fn test_dupes_command_invalid_tree() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["dupes", "/tmp"])
            .assert()
            .failure()
            .stderr("ERROR: /tmp/tracks not found\n");
    }

    #[test]
    #[ignore]
    fn test_dupes_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("dupes")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
