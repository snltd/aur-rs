#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::{fixture, sample_output};
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_flac2mp3_command() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/flac2mp3"), &["01.tester.flac2mp3.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.flac2mp3.flac");
        let expected_file = tmp.path().join("01.tester.flac2mp3.mp3");

        assert!(!expected_file.exists());

        cargo_bin_cmd!("aur")
            .arg("flac2mp3")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(format!(
                "{file_under_test}\n  {}\n",
                file_under_test.file_name().unwrap(),
            ));

        assert!(file_under_test.exists());
        assert!(expected_file.exists());

        cargo_bin_cmd!("aur")
            .arg("info")
            .arg(&expected_file)
            .assert()
            .success()
            .stdout(sample_output("commands/flac2mp3/transcoded_info"));

        cargo_bin_cmd!("aur")
            .arg("flac2mp3")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stdout(format!(
                "{file_under_test}\n  target exists ({expected_file})\n",
            ));
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_command_mp3() {
        let file_under_test = fixture("commands/flac2mp3/01.tester.test_no-op.mp3");

        cargo_bin_cmd!("aur")
            .arg("flac2mp3")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr("ERROR: Only FLAC files can be flac2mp3-ed\n");
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_command_missing_file() {
        cargo_bin_cmd!("aur")
            .arg("flac2mp3")
            .arg("/no/such/file.flac")
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("flac2mp3")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
