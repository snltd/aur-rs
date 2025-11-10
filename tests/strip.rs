#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use camino_tempfile_ext::prelude::*;
    use predicates::prelude::*;

    #[test]
    #[ignore]
    fn test_strip_command_flac() {
        let file_name = "01.tester.not_stripped.flac";
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/strip"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().canonicalize_utf8().unwrap().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("lint")
            .arg("--config")
            .arg(&fixture_as_string("config/test.toml"))
            .arg(&file_under_test)
            .assert()
            .failure()
            .stdout(predicate::str::contains("Unexpected tags: composer, tempo"))
            .stdout(predicate::str::contains("File contains embedded artwork"));

        cargo_bin_cmd!("aur")
            .arg("strip")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Strip: {file_under_test} :: composer, encoder, tempo"
            )))
            .stdout(predicate::str::contains(format!(
                "Strip: {file_under_test} :: embedded artwork"
            )));

        cargo_bin_cmd!("aur")
            .arg("lint")
            .arg("--config")
            .arg(&fixture_as_string("config/test.toml"))
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_strip_command_mp3() {
        let file_name = "02.tester.not_stripped.mp3";

        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/strip"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().canonicalize_utf8().unwrap().join(file_name);

        cargo_bin_cmd!("aur")
            .arg("lint")
            .arg("--config")
            .arg(&fixture_as_string("config/test.toml"))
            .arg(&file_under_test)
            .assert()
            .failure()
            .stdout(predicate::str::contains(
                "Unexpected tags: apic, tcom, tenc, txxx",
            ))
            .stdout(predicate::str::contains("File contains embedded artwork"));

        cargo_bin_cmd!("aur")
            .arg("strip")
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Strip: {file_under_test} :: apic, tcom, tenc, tlen, tsse, txxx"
            )))
            .stdout(predicate::str::contains(format!(
                "Strip: {file_under_test} :: embedded artwork"
            )));

        cargo_bin_cmd!("aur")
            .arg("lint")
            .arg("--config")
            .arg(&fixture_as_string("config/test.toml"))
            .arg(&file_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_strip_incorrect_usage() {
        cargo_bin_cmd!("aur")
            .arg("strip")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
