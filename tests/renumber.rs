#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_renumber_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/renumber"), &["*"]).unwrap();
        let file_01_step_1 = tmp.path().join("01.test.song.flac");
        let file_02_step_1 = tmp.path().join("02.test.song.mp3");
        let file_01_step_2 = tmp.path().join("15.test.song.flac");
        let file_02_step_2 = tmp.path().join("16.test.song.mp3");

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "renumber",
                "up",
                "14",
                &file_01_step_1.to_string_lossy(),
                &file_02_step_1.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "01.test.song.flac -> 15.test.song.flac",
            ))
            .stdout(predicate::str::contains("t_num -> 15"))
            .stdout(predicate::str::contains("t_num -> 16"))
            .stdout(predicate::str::contains(
                "02.test.song.mp3 -> 16.test.song.mp3",
            ));

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "renumber",
                "down",
                "7",
                &file_01_step_2.to_string_lossy(),
                &file_02_step_2.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "15.test.song.flac -> 08.test.song.flac",
            ))
            .stdout(predicate::str::contains("t_num -> 8"))
            .stdout(predicate::str::contains("t_num -> 9"))
            .stdout(predicate::str::contains(
                "16.test.song.mp3 -> 09.test.song.mp3",
            ));
    }

    #[test]
    #[ignore]
    fn test_renumber_command_bad_input() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/renumber"), &["02.test.song.mp3"])
            .unwrap();
        let file_under_test = tmp.path().join("02.test.song.mp3");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["renumber", &file_under_test.to_string_lossy()])
            .assert()
            .failure()
            .stderr(predicate::str::contains("[possible values: up, down]"));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["renumber", "up", "1000", &file_under_test.to_string_lossy()])
            .assert()
            .failure()
            .stderr("ERROR: Delta must be from 1 to 99 inclusive\n");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["renumber", "down", "30", &file_under_test.to_string_lossy()])
            .assert()
            .failure()
            .stderr("ERROR: Tag number must be from 1 to 99 inclusive\n");
    }

    #[test]
    #[ignore]
    fn test_renumber_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["renumber", "up", "2", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_renumber_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("renumber")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
