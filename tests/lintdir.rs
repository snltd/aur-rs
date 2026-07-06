#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use predicates::prelude::*;
    use snltest::fixture;

    #[test]
    #[ignore]
    fn test_lintdir_command_okay() {
        cargo_bin_cmd!("aur")
            .arg("lintdir")
            .arg(fixture!("commands/lintdir/flac/tester.perfect"))
            .arg(fixture!("commands/lintdir/mp3/tester.perfect"))
            .arg(fixture!("commands/lintdir/mp3/artist--band.split_single"))
            .arg(fixture!("commands/lintdir/mp3/various.compilation"))
            .arg(fixture!("commands/lintdir/mp3/tester.perfect--featuring"))
            .arg(fixture!("commands/lintdir/mp3/tester.bonus_disc"))
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_artwork_too_small() {
        let test_dir = fixture!("commands/lintdir/flac/tester.artwork_too_small");

        cargo_bin_cmd!("aur")
            .arg("lintdir")
            .arg(&test_dir)
            .assert()
            .failure()
            .stdout(output(test_dir.as_str(), "Cover art is too small"));
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_artwork_too_big() {
        let test_dir = &fixture!("commands/lintdir/flac/tester.artwork_too_big");

        cargo_bin_cmd!("aur")
            .arg("lintdir")
            .arg(test_dir)
            .assert()
            .failure()
            .stdout(output(test_dir.as_str(), "Cover art is too big"));
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_artwork_not_square() {
        let test_dir = &fixture!("commands/lintdir/flac/tester.artwork_not_square");

        cargo_bin_cmd!("aur")
            .arg("lintdir")
            .arg(test_dir)
            .assert()
            .failure()
            .stdout(output(test_dir.as_str(), "Cover art is not square"));
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_unwanted_art() {
        let test_dir = &fixture!("commands/lintdir/mp3/tester.unwanted_art");
        cargo_bin_cmd!("aur")
            .arg("lintdir")
            .arg(test_dir)
            .assert()
            .failure()
            .stdout(output(
                test_dir.as_str(),
                &format!("Unexpected file(s): {test_dir}/front.jpg"),
            ));
    }

    #[test]
    #[ignore]
    fn test_lintdir_command_mixed_genre() {
        let test_dir = &fixture!("commands/lintdir/mp3/tester.mixed_genre_year_album");

        cargo_bin_cmd!("aur")
            .arg("lintdir")
            .arg(test_dir)
            .assert()
            .failure()
            .stdout(output(
                test_dir.as_str(),
                "Inconsistent tags: album, genre, year",
            ));
    }

    #[test]
    #[ignore]
    fn test_lintdir_incorrect_usage() {
        cargo_bin_cmd!("aur")
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
