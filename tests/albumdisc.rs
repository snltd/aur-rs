#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    const FILENAME: &str = "01.artist.song.mp3";

    #[ignore]
    #[test]
    fn test_albumdisc_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.child("album/disc_3").create_dir_all().unwrap();
        let target = tmp.child("album/disc_3");
        target
            .copy_from(fixture("commands/albumdisc/disc_3/"), &[FILENAME])
            .unwrap();

        let file_under_test = target.path().join(FILENAME);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["albumdisc", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(predicate::str::ends_with("album -> Test Album (Disc 3)\n"));

        // Running again should do nothing, because it's been corrected
        Command::cargo_bin("aur")
            .unwrap()
            .args(["albumdisc", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[ignore]
    #[test]
    fn test_albumdisc_file_not_in_disc_directory() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/albumdisc/disc_3"), &[FILENAME])
            .unwrap();
        let file_under_test = tmp.path().join(FILENAME);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["albumdisc", &file_under_test.to_string_lossy()])
            .assert()
            .failure()
            .stderr(predicate::str::ends_with("is not in a disc_n directory\n"));
    }

    #[test]
    #[ignore]
    fn test_albumdisc_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("albumdisc")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
