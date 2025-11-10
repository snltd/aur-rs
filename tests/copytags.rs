#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_copytags_command() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/copytags"), &["**/*"])
            .unwrap();
        let file_under_test = tmp.path().join("mp3").join("01.artist.song.mp3");

        // Check the title is what we think it is (and wrong)
        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("title")
            .arg("--short")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("Wrong Title\n");

        // Copy the tags
        cargo_bin_cmd!("aur")
            .arg("copytags")
            .arg("--force")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains("title -> Right Title"))
            .stdout(predicate::str::contains("album -> Copytags Test"))
            .stdout(predicate::str::contains("t_num -> 1"))
            .stdout(predicate::str::contains("year -> 2021"));

        // Check the title is now correct
        cargo_bin_cmd!("aur")
            .arg("get")
            .arg("title")
            .arg("--short")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("Right Title\n");

        // This time nothing should happen because the tags already match
        cargo_bin_cmd!("aur")
            .arg("copytags")
            .arg("--force")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_copytags_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("copytags")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
