#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_retitle_command() {
        let file_name = "02.test_artist.this_title_needs_sorting.flac";

        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/retitle"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["retitle", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(predicate::str::contains("artist -> Test Artist"))
            .stdout(predicate::str::contains("album -> Test the Retitle"))
            .stdout(predicate::str::contains(
                "title -> This Title Needs Sorting",
            ));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["retitle", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_retitle_command_bad_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["retitle", &fixture_as_string("info/bad_file.flac")])
            .assert()
            .failure()
            .stderr("ERROR: InvalidInput: reader does not contain flac metadata\n");
    }

    #[test]
    #[ignore]
    fn test_retitle_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["retitle", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_retitle_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("retitle")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
