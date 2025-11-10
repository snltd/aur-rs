#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture_as_string;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_verify_command_some_valid_some_not() {
        let dir_under_test = fixture_as_string("commands/verify");

        cargo_bin_cmd!("aur")
            .arg("verify")
            .arg("-r")
            .arg(&dir_under_test)
            .assert()
            .failure()
            .stdout(predicate::str::contains("04.tester.truncated.mp3"))
            .stdout(predicate::str::contains("02.tester.truncated.flac"))
            .stdout(predicate::str::contains("06.tester.junk.mp3"))
            .stdout(predicate::str::contains("05.tester.junk.flac"))
            .stdout(predicate::str::contains("OK").not());

        cargo_bin_cmd!("aur")
            .arg("verify")
            .arg("-r")
            .arg("-v")
            .arg(&dir_under_test)
            .assert()
            .failure()
            .stdout(predicate::str::contains("04.tester.truncated.mp3"))
            .stdout(predicate::str::contains("02.tester.truncated.flac"))
            .stdout(predicate::str::contains("06.tester.junk.mp3"))
            .stdout(predicate::str::contains("05.tester.junk.flac"))
            .stdout(predicate::str::contains("OK"))
            .stdout(predicate::str::contains("01.tester.valid.flac"));
    }

    #[test]
    #[ignore]
    fn test_verify_command_valid_file() {
        cargo_bin_cmd!("aur")
            .arg("verify")
            .arg("-r")
            .arg(fixture_as_string("commands/verify/01.tester.valid.flac"))
            .assert()
            .success();
    }

    #[test]
    #[ignore]
    fn test_verify_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("verify")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
