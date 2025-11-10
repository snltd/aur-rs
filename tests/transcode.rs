#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_transcode_command_wav() {
        let file_name = "01.tester.lossless.wav";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/transcode"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let expected_file = tmp.path().join("01.tester.lossless.flac");

        assert!(!expected_file.exists());

        cargo_bin_cmd!("aur")
            .arg("transcode")
            .arg("--verbose")
            .arg("flac")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(format!("{file_under_test} -> {expected_file}\n"));

        assert!(expected_file.exists());

        cargo_bin_cmd!("aur")
            .arg("transcode")
            .arg("--verbose")
            .arg("flac")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stdout(format!(
                "target '{expected_file}' exists. Use -f to overwrite\n",
            ));
    }

    #[test]
    #[ignore]
    fn test_transcode_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("transcode")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        cargo_bin_cmd!("aur")
            .args(["transcode", "flac"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
