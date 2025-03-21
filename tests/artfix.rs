#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

    #[ignore]
    #[test]
    fn test_artfix_link() {
        let source = assert_fs::TempDir::new().unwrap();
        let linkdir = assert_fs::TempDir::new().unwrap();
        source
            .copy_from(fixture("commands/artfix"), &["tester.all_wrong/*"])
            .unwrap();
        let dir_under_test = source.join("tester.all_wrong");

        assert!(dir_under_test.join("some_file.JPEG").exists());
        assert!(!dir_under_test.join("front.jpg").exists());

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "artfix",
                "-d",
                &linkdir.to_string_lossy(),
                &dir_under_test.to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Rename: {dir}/some_file.JPEG -> front.jpg",
                dir = dir_under_test.display()
            )));

        assert!(!dir_under_test.join("some_file.JPEG").exists());
        assert!(dir_under_test.join("front.jpg").exists());

        Command::new("ls")
            .args(["-l", &linkdir.to_string_lossy()])
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "-tester.all_wrong-front.jpg -> {}/front.jpg",
                dir_under_test.to_string_lossy()
            )));
    }

    #[ignore]
    #[test]
    fn test_artfix_scale() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/artfix"), &["tester.too_big/*"])
            .unwrap();
        let dir_under_test = tmp.join("tester.too_big");

        Command::cargo_bin("aur")
            .unwrap()
            .args(["artfix", &dir_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Resize: {}/front.jpg -> 750x750",
                dir_under_test.display()
            )));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["artfix", &dir_under_test.to_string_lossy()])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_artfix_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("artfix")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
