#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_name2num_command() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("name2num")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("           t_num -> 1\n");

        cargo_bin_cmd!("aur")
            .arg("name2num")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_name2num_command_bad_file() {
        cargo_bin_cmd!("aur")
            .args(["name2num", &fixture_as_string("info/bad_file.flac")])
            .assert()
            .failure()
            .stderr("ERROR: InvalidInput: reader does not contain flac metadata\n");
    }

    #[test]
    #[ignore]
    fn test_name2num_command_missing_file() {
        cargo_bin_cmd!("aur")
            .args(["name2num", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_name2num_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("name2num")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
