#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_copytags_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/copytags"), &["**/*"])
            .unwrap();
        let file_under_test = tmp.path().join("mp3").join("01.artist.song.mp3");

        // Check the title is what we think it is (and wrong)
        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "get",
                "title",
                "--short",
                &file_under_test.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout("Wrong Title\n");

        // Copy the tags
        Command::cargo_bin("aur")
            .unwrap()
            .args(["copytags", "--force", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(predicate::str::contains("title -> Right Title"))
            .stdout(predicate::str::contains("album -> Copytags Test"))
            .stdout(predicate::str::contains("t_num -> 1"))
            .stdout(predicate::str::contains("year -> 2021"));

        // Check the title is now correct
        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "get",
                "title",
                "--short",
                &file_under_test.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout("Right Title\n");

        // This time nothing should happen because the tags already match
        Command::cargo_bin("aur")
            .unwrap()
            .args(["copytags", "--force", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    // I'm not sure at the moment whether I want this behaviour or not.
    // #[test]
    // #[ignore]
    // fn test_copytags_command_no_partner() {
    //     let tmp = assert_fs::TempDir::new().unwrap();
    //     tmp.copy_from(fixture("commands/copytags"), &["**/*"])
    //         .unwrap();
    //     let file_under_test = tmp.path().join("mp3").join("02.artist.song.mp3");

    //     // Should fail because there's no corresponding FLAC
    //     Command::cargo_bin("aur")
    //         .unwrap()
    //         .args(["copytags", "--force", &file_under_test.to_string_lossy()])
    //         .assert()
    //         .failure()
    //         .stdout("")
    //         .stderr(predicate::str::contains(
    //             "has no partner from which to copy tags",
    //         ));
    // }

    #[test]
    #[ignore]
    fn test_copytags_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("copytags")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
