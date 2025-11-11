#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture_as_string;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_namecheck_command_valid_tree() {
        cargo_bin_cmd!("aur")
            .arg("namecheck")
            .arg(fixture_as_string("commands/namecheck"))
            .assert()
            .failure()
            .stdout(predicate::str::contains("Artist"))
            .stdout(predicate::str::contains("The B-52's"));
    }

    #[test]
    #[ignore]
    fn test_namecheck_command_invalid_tree() {
        cargo_bin_cmd!("aur")
            .args(["namecheck", "/no/such/dir"])
            .assert()
            .failure()
            .stderr("ERROR: No files found\n");
    }

    #[test]
    #[ignore]
    fn test_namecheck_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("namecheck")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
