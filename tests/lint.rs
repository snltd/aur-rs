#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use aur::test_utils::spec_helper::fixture_as_string;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_lint_command_missing_tags() {
        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "lint",
                &fixture_as_string("commands/lint/00.tester.missing_genre_track_no_year.flac"),
            ])
            .assert()
            .failure()
            .stdout(predicate::str::contains("Invalid track number tag: 0"))
            .stdout(predicate::str::contains("Invalid year tag: 0"))
            .stdout(predicate::str::contains("Invalid genre tag: "));
    }

    #[test]
    #[ignore]
    fn test_lint_command_fine() {
        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "lint",
                &fixture_as_string("commands/lint/01.tester.lints_fine.flac"),
                &fixture_as_string("commands/lint/02.tester.lints_fine.mp3"),
            ])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_lint_command_respects_config() {
        let file_to_test = fixture_as_string("commands/lint/09.tester.bad_title_allowed.mp3");
        Command::cargo_bin("aur")
            .unwrap()
            .args(["lint", &file_to_test])
            .assert()
            .failure()
            .stdout(predicate::str::contains(
                "Invalid title tag: BAD title allo,Wed",
            ));

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "lint",
                "--config",
                &fixture_as_string("config/test.toml"),
                &file_to_test,
            ])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_lint_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("lint")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
