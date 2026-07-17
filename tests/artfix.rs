#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::utils::config::ARTWORK_FILENAME;
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;
    use snltest::fixture;

    #[ignore]
    #[test]
    fn test_artfix_link() {
        let source = Utf8TempDir::new().unwrap();
        let linkdir = Utf8TempDir::new().unwrap();

        source
            .copy_from(fixture!("commands/artfix"), &["tester.all_wrong/*"])
            .unwrap();

        let dir_under_test = source.path().join("tester.all_wrong");

        assert!(dir_under_test.join("some_file.JPEG").exists());
        assert!(!dir_under_test.join(ARTWORK_FILENAME).exists());

        cargo_bin_cmd!("aur")
            .arg("artfix")
            .arg("-d")
            .arg(linkdir.path())
            .arg(&dir_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Rename: {dir_under_test}/some_file.JPEG -> {ARTWORK_FILENAME}",
            )));

        assert!(!dir_under_test.join("some_file.JPEG").exists());
        assert!(dir_under_test.join(ARTWORK_FILENAME).exists());

        Command::new("ls")
            .arg("-l")
            .arg(linkdir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "-tester.all_wrong-cover.jpg -> {dir_under_test}/{ARTWORK_FILENAME}",
            )));
    }

    #[ignore]
    #[test]
    fn test_artfix_scale() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture!("commands/artfix"), &["tester.too_big/*"])
            .unwrap();
        let dir_under_test = tmp.path().join("tester.too_big");

        cargo_bin_cmd!("aur")
            .arg("artfix")
            .arg(&dir_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Resize: {dir_under_test}/{ARTWORK_FILENAME} -> 750x750",
            )));

        cargo_bin_cmd!("aur")
            .arg("artfix")
            .arg(&dir_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_artfix_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("artfix")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
