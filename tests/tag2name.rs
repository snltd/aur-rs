#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_tag2name_command() {
        let file_name = "badly_named_file.mp3";

        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tag2name"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("tag2name")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("  badly_named_file.mp3 -> 01.tester.some_song--or_other.mp3\n");

        let renamed_file = tmp.path().join("01.tester.some_song--or_other.mp3");

        cargo_bin_cmd!("aur")
            .arg("tag2name")
            .arg(&renamed_file)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_tag2name_command_missing_file() {
        cargo_bin_cmd!("aur")
            .arg("tag2name")
            .arg("/no/such/file.flac")
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_tag2name_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("tag2name")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
