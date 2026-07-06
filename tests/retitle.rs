#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;
    use snltest::fixture;

    #[test]
    #[ignore]
    fn test_retitle_command() {
        let file_name = "02.test_artist.this_title_needs_sorting.flac";

        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/retitle"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("retitle")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains("artist -> Test Artist"))
            .stdout(predicate::str::contains("album -> Test the Retitle"))
            .stdout(predicate::str::contains(
                "title -> This Title Needs Sorting",
            ));

        cargo_bin_cmd!("aur")
            .arg("retitle")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_retitle_command_bad_file() {
        cargo_bin_cmd!("aur")
            .arg("retitle")
            .arg(fixture!("info/bad_file.flac"))
            .assert()
            .failure()
            .stderr(predicate::str::starts_with("Error tagging"))
            .stderr(predicate::str::ends_with(
                "InvalidInput: reader does not contain flac metadata\n",
            ));
    }

    #[test]
    #[ignore]
    fn test_retitle_command_missing_file() {
        cargo_bin_cmd!("aur")
            .args(["retitle", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("Error tagging /no/such/file.flac: No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_retitle_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("retitle")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
