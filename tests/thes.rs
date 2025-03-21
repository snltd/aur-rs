#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_thes_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/thes"), &["*"]).unwrap();
        let file_under_test = tmp.path().join("01.tester.song.mp3");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["thes", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("          artist -> The Tester\n");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["thes", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_thes_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["thes", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_thes_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("thes")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
