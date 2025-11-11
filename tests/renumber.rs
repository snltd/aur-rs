#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_renumber_command() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/renumber"), &["*"]).unwrap();
        let file_01_step_1 = tmp.path().join("01.test.song.flac");
        let file_02_step_1 = tmp.path().join("02.test.song.mp3");
        let file_01_step_2 = tmp.path().join("15.test.song.flac");
        let file_02_step_2 = tmp.path().join("16.test.song.mp3");

        cargo_bin_cmd!("aur")
            .arg("renumber")
            .arg("up")
            .arg("14")
            .arg(&file_01_step_1)
            .arg(&file_02_step_1)
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

        cargo_bin_cmd!("aur")
            .arg("renumber")
            .arg("down")
            .arg("7")
            .arg(&file_01_step_2)
            .arg(&file_02_step_2)
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
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/renumber"), &["02.test.song.mp3"])
            .unwrap();
        let file_under_test = tmp.path().join("02.test.song.mp3");

        cargo_bin_cmd!("aur")
            .arg("renumber")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr(predicate::str::contains("[possible values: up, down]"));

        cargo_bin_cmd!("aur")
            .arg("renumber")
            .arg("up")
            .arg("1000")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr("ERROR: Delta must be from 1 to 99 inclusive\n");

        cargo_bin_cmd!("aur")
            .arg("renumber")
            .arg("down")
            .arg("30")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stderr("ERROR: Tag number must be from 1 to 99 inclusive\n");
    }

    #[test]
    #[ignore]
    fn test_renumber_command_missing_file() {
        cargo_bin_cmd!("aur")
            .args(["renumber", "up", "2", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_renumber_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("renumber")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
