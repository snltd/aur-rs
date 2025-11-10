#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    const TEST_FILE: &str = "01.artist.song.mp3";

    #[ignore]
    #[test]
    fn test_albumdisc_command() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.child("album/disc_3").create_dir_all().unwrap();

        let target = tmp.child("album/disc_3");
        target
            .copy_from(fixture("commands/albumdisc/disc_3/"), &[TEST_FILE])
            .unwrap();

        let file_under_test = target.join(TEST_FILE);

        cargo_bin_cmd!("aur")
            .arg("albumdisc")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(predicate::str::ends_with("album -> Test Album (Disc 3)\n"));

        // Running again should do nothing, because it's been corrected
        cargo_bin_cmd!("aur")
            .arg("albumdisc")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[ignore]
    #[test]
    fn test_albumdisc_file_not_in_disc_directory() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/albumdisc/disc_3"), &[TEST_FILE])
            .unwrap();

        let file_under_test = tmp.path().join(TEST_FILE);

        cargo_bin_cmd!("aur")
            .arg("albumdisc")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr(predicate::str::ends_with("is not in a disc_n directory\n"));
    }

    #[test]
    #[ignore]
    fn test_albumdisc_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("albumdisc")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
