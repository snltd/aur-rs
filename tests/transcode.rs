#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_transcode_command_wav() {
        let file_name = "01.tester.lossless.wav";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/transcode"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();
        let expected_file = tmp.path().join("01.tester.lossless.flac");

        assert!(!expected_file.exists());

        Command::cargo_bin("aur")
            .unwrap()
            .args(["transcode", "--verbose", "flac", &file_str])
            .assert()
            .success()
            .stdout(format!(
                "{} -> {}\n",
                file_under_test.display(),
                expected_file.display(),
            ));

        assert!(expected_file.exists());

        Command::cargo_bin("aur")
            .unwrap()
            .args(["transcode", "--verbose", "flac", &file_str])
            .assert()
            .success()
            .stdout(format!(
                "target '{}' exists. Use -f to overwrite\n",
                expected_file.display()
            ));
    }

    #[test]
    #[ignore]
    fn test_transcode_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("transcode")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["transcode", "flac"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
