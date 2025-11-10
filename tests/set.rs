#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_set_command() {
        let file_name = "02.tester.song.mp3";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/set"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("set")
            .arg("title")
            .arg("New Title")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("           title -> New Title\n");

        cargo_bin_cmd!("aur")
            .arg("set")
            .arg("title")
            .arg("New Title")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_set_command_missing_file() {
        cargo_bin_cmd!("aur")
            .arg("set")
            .arg("title")
            .arg("New Title")
            .arg("/no/such/file.flac")
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_set_command_bad_input() {
        let file_name = "02.tester.song.mp3";

        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/set"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("set")
            .arg("whatever")
            .arg("New Title")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr("ERROR: Unknown tag name\n");

        cargo_bin_cmd!("aur")
            .arg("set")
            .arg("t_num")
            .arg("five")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr("ERROR: (Parsing): invalid digit found in string\n");
    }

    #[test]
    #[ignore]
    fn test_set_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("set")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));

        cargo_bin_cmd!("aur")
            .args(["set", "title"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
