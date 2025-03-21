#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use aur::test_utils::spec_helper::fixture_as_string;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_lintdir_command_okay() {
        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "lintdir",
                &fixture_as_string("commands/lintdir/flac/tester.perfect"),
                &fixture_as_string("commands/lintdir/mp3/tester.perfect"),
                &fixture_as_string("commands/lintdir/mp3/artist--band.split_single"),
                &fixture_as_string("commands/lintdir/mp3/various.compilation"),
                &fixture_as_string("commands/lintdir/mp3/tester.perfect--featuring"),
                &fixture_as_string("commands/lintdir/mp3/tester.bonus_disc"),
            ])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_artwork_too_small() {
        let test_dir = &fixture_as_string("commands/lintdir/flac/tester.artwork_too_small");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["lintdir", test_dir])
            .assert()
            .success()
            .stdout(output(test_dir, "Cover art is too small"));
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_artwork_too_big() {
        let test_dir = &fixture_as_string("commands/lintdir/flac/tester.artwork_too_big");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["lintdir", test_dir])
            .assert()
            .success()
            .stdout(output(test_dir, "Cover art is too big"));
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_artwork_not_square() {
        let test_dir = &fixture_as_string("commands/lintdir/flac/tester.artwork_not_square");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["lintdir", test_dir])
            .assert()
            .success()
            .stdout(output(test_dir, "Cover art is not square"));
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_unwanted_art() {
        let test_dir = &fixture_as_string("commands/lintdir/mp3/tester.unwanted_art");
        Command::cargo_bin("aur")
            .unwrap()
            .args(["lintdir", test_dir])
            .assert()
            .success()
            .stdout(output(
                test_dir,
                format!("Unexpected file(s): {}/front.jpg", test_dir).as_str(),
            ));
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_mixed_genre() {
        let test_dir = &fixture_as_string("commands/lintdir/mp3/tester.mixed_genre_year_album");
        Command::cargo_bin("aur")
            .unwrap()
            .args(["lintdir", test_dir])
            .assert()
            .success()
            .stdout(output(test_dir, "Inconsistent tags: album, genre, year"));
    }

    #[test]
    #[ignore]
    fn test_lintdir_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("lintdir")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }

    fn output(file: &str, message: &str) -> String {
        format!("{}\n  {}\n\n", file, message)
    }
}
