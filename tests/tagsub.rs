#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_tagsub_command_change() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();

        Command::cargo_bin("aur")
            .unwrap()
            .args(["--verbose", "tagsub", "artist", "Test", "Tested", &file_str])
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "06.test_artist.test_title.mp3: Test Artist -> Tested Artist",
            ))
            .stdout(predicate::str::contains("artist -> Tested Artist"));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "--short", "artist", &file_str])
            .assert()
            .success()
            .stdout("Tested Artist\n");
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_noop() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();

        Command::cargo_bin("aur")
            .unwrap()
            .args(["--noop", "tagsub", "artist", "Test", "Tested", &file_str])
            .assert()
            .success()
            .stdout(format!("{}: Test Artist -> Tested Artist\n", file_str));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "--short", "artist", &file_str])
            .assert()
            .success()
            .stdout("Test Artist\n");
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["tagsub", "title", "find", "replace", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_bad_input() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();

        Command::cargo_bin("aur")
            .unwrap()
            .args(["tagsub", "whatever", "find", "replace", &file_str])
            .assert()
            .failure()
            .stderr("ERROR: Unknown tag: whatever\n");
    }

    #[test]
    #[ignore]
    fn test_tagsub_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("tagsub")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
