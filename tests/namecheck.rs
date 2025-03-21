#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use aur::test_utils::spec_helper::fixture_as_string;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_namecheck_command_valid_tree() {
        // this test is rubbish. make it better
        let dir_under_test = fixture_as_string("commands/namecheck");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["namecheck", &dir_under_test])
            .assert()
            .success()
            .stdout(predicate::str::contains("Artist"))
            .stdout(predicate::str::contains("The B-52's"));
    }

    #[test]
    #[ignore]
    fn test_namecheck_command_invalid_tree() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["namecheck", "/no/such/dir"])
            .assert()
            .failure()
            .stderr("ERROR: No files found\n");
    }

    #[test]
    #[ignore]
    fn test_namecheck_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("namecheck")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
