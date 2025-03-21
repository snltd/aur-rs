#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, sample_output};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_flac2mp3_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/flac2mp3"), &["01.tester.flac2mp3.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.flac2mp3.flac");
        let expected_file = tmp.path().join("01.tester.flac2mp3.mp3");

        assert!(!expected_file.exists());

        Command::cargo_bin("aur")
            .unwrap()
            .args(["flac2mp3", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(format!(
                "{}\n  {}\n",
                file_under_test.display(),
                file_under_test.file_name().unwrap().to_string_lossy(),
            ));

        assert!(file_under_test.exists());
        assert!(expected_file.exists());

        Command::cargo_bin("aur")
            .unwrap()
            .args(["info", &expected_file.to_string_lossy()])
            .assert()
            .success()
            .stdout(sample_output("commands/flac2mp3/transcoded_info"));

        // Probably should exit 1
        Command::cargo_bin("aur")
            .unwrap()
            .args(["flac2mp3", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(format!(
                "{}\n  target exists ({})\n",
                file_under_test.display(),
                expected_file.display(),
            ));
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_command_mp3() {
        let file_under_test = fixture("commands/flac2mp3/01.tester.test_no-op.mp3");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["flac2mp3", &file_under_test.into_string()])
            .assert()
            .success() //FIXME
            .stderr("ERROR: Only FLAC files can be flac2mp3-ed\n");
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["flac2mp3", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("flac2mp3")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
