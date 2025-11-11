#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_tagsub_command_change() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("--verbose")
            .arg("tagsub")
            .arg("artist")
            .arg("Test")
            .arg("Tested")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "06.test_artist.test_title.mp3: Test Artist -> Tested Artist",
            ))
            .stdout(predicate::str::contains("artist -> Tested Artist"));

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("--short")
            .arg("artist")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("Tested Artist\n");
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_noop() {
        let file_name = "06.test_artist.test_title.mp3";

        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("--noop")
            .arg("tagsub")
            .arg("artist")
            .arg("Test")
            .arg("Tested")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(format!("{file_under_test}: Test Artist -> Tested Artist\n"));

        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("--short")
            .arg("artist")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("Test Artist\n");
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_missing_file() {
        cargo_bin_cmd!("aur")
            .arg("tagsub")
            .arg("title")
            .arg("find")
            .arg("replace")
            .arg("/no/such/file.flac")
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_tagsub_command_bad_input() {
        let file_name = "06.test_artist.test_title.mp3";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/tagsub"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("tagsub")
            .arg("whatever")
            .arg("find")
            .arg("replace")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr("ERROR: Unknown tag: whatever\n");
    }

    #[test]
    #[ignore]
    fn test_tagsub_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("tagsub")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
