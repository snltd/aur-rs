#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_name2tag_command() {
        let file_name = "01.test_artist.untagged_song.flac";

        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.child("test.artist.test_album")
            .create_dir_all()
            .unwrap();

        let target = tmp.child("test_artist.test_album");

        target
            .copy_from(fixture("commands/name2tag"), &[file_name])
            .unwrap();

        let file_under_test = target.path().join(file_name);

        Command::cargo_bin("aur")
            .unwrap()
            .args(["name2tag", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(predicate::str::contains("t_num -> 1"))
            .stdout(predicate::str::contains("artist -> Test Artist"))
            .stdout(predicate::str::contains("album -> Test Album"))
            .stdout(predicate::str::contains("title -> Untagged Song"));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["name2tag", &file_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_name2tag_command_bad_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["name2tag", &fixture_as_string("info/bad_file.flac")])
            .assert()
            .failure()
            .stderr("ERROR: InvalidInput: reader does not contain flac metadata\n");
    }

    #[test]
    #[ignore]
    fn test_name2tag_command_missing_file() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["name2tag", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_name2tag_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("name2tag")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
