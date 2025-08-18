#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, sample_output};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_syncflac_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands"), &["syncflac/**/*"])
            .unwrap();
        let dir_under_test = tmp.path().canonicalize().unwrap().join("syncflac");

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "syncflac",
                "--verbose",
                "-R",
                &dir_under_test.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("Creating target"))
            .stdout(predicate::str::contains(format!(
                "Removing {}",
                tmp.path()
                    .canonicalize()
                    .unwrap()
                    .join("syncflac/mp3/eps/band.flac_and_mp3_unequal/03.band.song_3.mp3")
                    .display()
            )));

        Command::new("ls")
            .current_dir(&dir_under_test)
            .arg("-R")
            .assert()
            .success()
            .stdout(sample_output("commands/syncflac/ls"));

        let sample_file = dir_under_test
            .join("mp3/albums/tuv/tester.flac_album")
            .join("01.tester.song_1.mp3");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["info", &sample_file.to_string_lossy()])
            .assert()
            .success()
            .stdout(sample_output("commands/syncflac/info-new-mp3"));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["syncflac", "-R", &dir_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_syncflac_bad_directory() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["syncflac", "--root", "/usr"])
            .assert()
            .failure()
            .stdout("")
            .stderr("ERROR: did not find /usr/mp3\n");
    }
}
