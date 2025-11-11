#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_name2tag_command() {
        let file_name = "01.test_artist.untagged_song.flac";

        let tmp = Utf8TempDir::new().unwrap();

        tmp.child("test.artist.test_album")
            .create_dir_all()
            .unwrap();

        let target = tmp.child("test_artist.test_album");

        target
            .copy_from(fixture("commands/name2tag"), &[file_name])
            .unwrap();

        let file_under_test = target.join(file_name);

        cargo_bin_cmd!("aur")
            .arg("name2tag")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains("t_num -> 1"))
            .stdout(predicate::str::contains("artist -> Test Artist"))
            .stdout(predicate::str::contains("album -> Test Album"))
            .stdout(predicate::str::contains("title -> Untagged Song"));

        cargo_bin_cmd!("aur")
            .arg("name2tag")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_name2tag_command_bad_file() {
        cargo_bin_cmd!("aur")
            .arg("name2tag")
            .arg(&fixture_as_string("info/bad_file.flac"))
            .assert()
            .failure()
            .stderr("ERROR: InvalidInput: reader does not contain flac metadata\n");
    }

    #[test]
    #[ignore]
    fn test_name2tag_command_missing_file() {
        cargo_bin_cmd!("aur")
            .args(["name2tag", "/no/such/file.flac"])
            .assert()
            .failure()
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }

    #[test]
    #[ignore]
    fn test_name2tag_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("name2tag")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
