#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::fixture;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_dupes_command_valid_tree() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/dupes"), &["flac/**/*"])
            .unwrap();
        let file_under_test = tmp.path().join("flac");

        cargo_bin_cmd!("aur")
            .arg("dupes")
            .arg(&file_under_test)
            .assert()
            .failure()
            .stdout(predicate::str::contains(
                "flac/tracks/fall.free_ranger.flac",
            ))
            .stdout(predicate::str::contains("flac/tracks/slint.don_aman.flac"));
    }

    #[test]
    #[ignore]
    fn test_dupes_command_invalid_tree() {
        cargo_bin_cmd!("aur")
            .arg("dupes")
            .arg("/tmp")
            .assert()
            .failure()
            .stderr("ERROR: /tmp/tracks not found\n");
    }

    #[test]
    #[ignore]
    fn test_dupes_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("dupes")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
