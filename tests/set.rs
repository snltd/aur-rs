#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_set_command() {
        let file_name = "02.tester.song.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/set"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "set",
                "title",
                "New Title",
                &file_under_test.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout("           title -> New Title\n");

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "set",
                "title",
                "New Title",
                &file_under_test.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_set_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["set", "title", "new title", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_set_command_bad_input() {
        let file_name = "02.tester.song.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/set"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "set",
                "whatever",
                "new title",
                &file_under_test.to_string_lossy(),
            ])
            .assert()
            .failure()
            .stderr("ERROR: Unknown tag name\n");

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "set",
                "t_num",
                "five",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .assert()
            .failure()
            .stderr("ERROR: (Parsing): invalid digit found in string\n");
    }

    #[test]
    #[ignore]
    fn test_set_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("set")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["set", "title"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
