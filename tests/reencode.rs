#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;
    use std::fs;

    #[test]
    #[ignore]
    fn test_reencode_command_flac_keep_original() {
        let file_name = "01.tester.song.flac";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/reencode"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let expected_file = tmp.path().join("01.tester.song.reencoded.flac");

        assert!(!expected_file.exists());

        cargo_bin_cmd!("aur")
            .arg("reencode")
            .arg("-k")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(format!("{file_under_test}\n"));

        assert!(file_under_test.exists());
        assert!(expected_file.exists());
    }

    #[test]
    #[ignore]
    fn test_reencode_command_mp3_overwrite_original() {
        let file_name = "02.tester.song.mp3";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/reencode"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let intermediate_file = tmp.path().join("02.tester.song.reencoded.mp3");

        assert!(!intermediate_file.exists());
        let original_size = fs::metadata(&file_under_test).unwrap().len();

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("-s")
            .arg("bitrate")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("320kbps\n");

        cargo_bin_cmd!("aur")
            .arg("reencode")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(format!("{file_under_test}\n"));

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("-s")
            .arg("bitrate")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("128kbps\n");

        let new_size = fs::metadata(&file_under_test).unwrap().len();

        assert!(file_under_test.exists());
        assert!(!intermediate_file.exists());
        assert!(original_size > new_size);
    }

    #[test]
    #[ignore]
    fn test_reencode_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("reencode")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
