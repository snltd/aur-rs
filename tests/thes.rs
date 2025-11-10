#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_thes_command() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/thes"), &["*"]).unwrap();
        let file_under_test = tmp.path().join("01.tester.song.mp3");

        cargo_bin_cmd!("aur")
            .arg("thes")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("          artist -> The Tester\n");

        cargo_bin_cmd!("aur")
            .arg("thes")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_thes_command_missing_file() {
        cargo_bin_cmd!("aur")
            .arg("thes")
            .arg("/no/such/file.flac")
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_thes_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("thes")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
