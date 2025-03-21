#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use aur::test_utils::spec_helper::fixture_as_string;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_verify_command_some_valid_some_not() {
        let dir_under_test = fixture_as_string("commands/verify");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["verify", "-r", &dir_under_test])
            .assert()
            .failure()
            .stdout(predicate::str::contains("04.tester.truncated.mp3"))
            .stdout(predicate::str::contains("02.tester.truncated.flac"))
            .stdout(predicate::str::contains("06.tester.junk.mp3"))
            .stdout(predicate::str::contains("05.tester.junk.flac"))
            .stdout(predicate::str::contains("OK").not());

        Command::cargo_bin("aur")
            .unwrap()
            .args(["verify", "-r", "-v", &dir_under_test])
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
        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "verify",
                "-r",
                fixture_as_string("commands/verify/01.tester.valid.flac").as_str(),
            ])
            .assert()
            .success();
    }

    #[test]
    #[ignore]
    fn test_verify_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("verify")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
