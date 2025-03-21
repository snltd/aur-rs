#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_num2name_command() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/num2name"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["num2name", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("  01.test_artist.test_title.flac -> 02.test_artist.test_title.flac\n");

        let renamed_file = tmp.path().join("02.test_artist.test_title.flac");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["num2name", &renamed_file.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_num2name_command_bad_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["num2name", &fixture_as_string("info/bad_file.flac")])
            .assert()
            .failure()
            .stderr("ERROR: InvalidInput: reader does not contain flac metadata\n");
    }

    #[test]
    #[ignore]
    fn test_num2name_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["num2name", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_num2name_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("num2name")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
