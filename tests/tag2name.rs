#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_tag2name_command() {
        let file_name = "badly_named_file.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tag2name"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["tag2name", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("  badly_named_file.mp3 -> 01.tester.some_song--or_other.mp3\n");

        let renamed_file = tmp.path().join("01.tester.some_song--or_other.mp3");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["tag2name", &renamed_file.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_tag2name_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["tag2name", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_tag2name_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("tag2name")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
