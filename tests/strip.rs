#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_strip_command_flac() {
        let file_name = "01.tester.not_stripped.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/strip"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "lint",
                "--config",
                &fixture_as_string("config/test.toml"),
                &file_str,
            ])
            .assert()
            .failure()
            .stdout(predicate::str::contains("Unexpected tags: composer, tempo"))
            .stdout(predicate::str::contains("File contains embedded artwork"));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["strip", &file_str])
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Strip: {} :: composer, encoder, tempo",
                file_under_test.display()
            )))
            .stdout(predicate::str::contains(
                format!("Strip: {} :: embedded artwork", file_under_test.display()).as_str(),
            ));

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "lint",
                "--config",
                &fixture_as_string("config/test.toml"),
                &file_str,
            ])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_strip_command_mp3() {
        let file_name = "02.tester.not_stripped.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/strip"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "lint",
                "--config",
                &fixture_as_string("config/test.toml"),
                &file_str,
            ])
            .assert()
            .failure()
            .stdout(predicate::str::contains(
                "Unexpected tags: apic, tcom, tenc, txxx",
            ))
            .stdout(predicate::str::contains("File contains embedded artwork"));

        Command::cargo_bin("aur")
            .unwrap()
            .args(["strip", &file_str])
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Strip: {} :: apic, tcom, tenc, tlen, tsse, txxx",
                file_under_test.display()
            )))
            .stdout(predicate::str::contains(format!(
                "Strip: {} :: embedded artwork",
                file_under_test.display()
            )));

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "lint",
                "--config",
                &fixture_as_string("config/test.toml"),
                &file_str,
            ])
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_strip_incorrect_usage() {
        Command::cargo_bin("aur")
            .unwrap()
            .arg("strip")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
