#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_cdq_command_overwrite() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["01.tester.hi-res.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.hi-res.flac");
        let cdq_file = tmp.path().join("01.tester.hi-res-cdq.flac");

        cargo_bin_cmd!("aur")
            .arg("cdq")
            .arg(&file_under_test)
            .assert()
            .stdout("")
            .success();

        assert!(!cdq_file.exists());

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("--short")
            .arg("bitrate")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("16-bit/44100Hz\n");
    }

    #[test]
    #[ignore]
    fn test_cdq_command_leave_original() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["01.tester.hi-res.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.hi-res.flac");
        let cdq_file = tmp.path().join("01.tester.hi-res-cdq.flac");

        cargo_bin_cmd!("aur")
            .arg("cdq")
            .arg("-l")
            .arg(&file_under_test)
            .assert()
            .success();

        assert!(cdq_file.exists());

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("bitrate")
            .arg("--short")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("24-bit/96000Hz\n");

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("-s")
            .arg("bitrate")
            .arg(cdq_file)
            .assert()
            .success()
            .stdout("16-bit/44100Hz\n");
    }

    #[test]
    #[ignore]
    fn test_cdq_command_mp3() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["02.tester.not_a_flac.mp3"])
            .unwrap();
        let file_under_test = tmp.path().join("02.tester.not_a_flac.mp3");

        cargo_bin_cmd!("aur")
            .arg("cdq")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr("ERROR: Only FLAC files can be CDQed\n");
    }

    #[test]
    #[ignore]
    fn test_cdq_command_missing_file() {
        cargo_bin_cmd!("aur")
            .arg("cdq")
            .arg("/no/such/file.flac")
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_cdq_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("cdq")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
