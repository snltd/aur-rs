#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_cdq_command_overwrite() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["01.tester.hi-res.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.hi-res.flac");
        let cdq_file = tmp.path().join("01.tester.hi-res-cdq.flac");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["cdq", &file_under_test.to_string_lossy()])
            .assert()
            .stdout("")
            .success();

        assert!(!cdq_file.exists());

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "get",
                "--short",
                "bitrate",
                &file_under_test.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout("16-bit/44100Hz\n");
    }

    #[test]
    #[ignore]
    fn test_cdq_command_leave_original() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["01.tester.hi-res.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.hi-res.flac");
        let cdq_file = tmp.path().join("01.tester.hi-res-cdq.flac");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["cdq", "-l", &file_under_test.to_string_lossy()])
            .assert()
            .success();

        assert!(cdq_file.exists());

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "get",
                "bitrate",
                "--short",
                &file_under_test.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout("24-bit/96000Hz\n");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["get", "-s", "bitrate", &cdq_file.to_string_lossy()])
            .assert()
            .success()
            .stdout("16-bit/44100Hz\n");
    }

    #[test]
    #[ignore]
    fn test_cdq_command_mp3() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["02.tester.not_a_flac.mp3"])
            .unwrap();
        let file_under_test = tmp.path().join("02.tester.not_a_flac.mp3");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["cdq", &file_under_test.to_string_lossy()])
            .assert()
            .failure()
            .stderr("ERROR: Only FLAC files can be CDQed\n");
    }

    #[test]
    #[ignore]
    fn test_cdq_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["cdq", "up", "2", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_cdq_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("cdq")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
