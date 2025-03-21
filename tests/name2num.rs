#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_name2num_command() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/name2num"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["name2num", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("           t_num -> 1\n");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["name2num", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_name2num_command_bad_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["name2num", &fixture_as_string("info/bad_file.flac")])
            .assert()
            .failure()
            .stderr("ERROR: InvalidInput: reader does not contain flac metadata\n");
    }

    #[test]
    #[ignore]
    fn test_name2num_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["name2num", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_name2num_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("name2num")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
