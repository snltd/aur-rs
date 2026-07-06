#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;
    use snltest::fixture;

    #[test]
    #[ignore]
    fn test_num2name_command() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/num2name"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("num2name")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("  01.test_artist.test_title.flac -> 02.test_artist.test_title.flac\n");

        let renamed_file = tmp.path().join("02.test_artist.test_title.flac");

        cargo_bin_cmd!("aur")
            .arg("num2name")
            .arg(&renamed_file)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_num2name_command_bad_file() {
        cargo_bin_cmd!("aur")
            .arg("num2name")
            .arg(fixture!("info/bad_file.flac"))
            .assert()
            .failure()
            .stderr(predicate::str::starts_with("Error renaming"))
            .stderr(predicate::str::ends_with(
                "InvalidInput: reader does not contain flac metadata\n",
            ));
    }

    #[test]
    #[ignore]
    fn test_num2name_command_missing_file() {
        cargo_bin_cmd!("aur")
            .args(["num2name", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("Error renaming /no/such/file.flac: No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_num2name_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("num2name")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
