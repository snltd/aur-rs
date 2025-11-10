#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::{config_file_as_string, fixture_as_string, sample_output};

    #[test]
    #[ignore]
    fn test_wantflac_command_valid_tree() {
        let dir_under_test = fixture_as_string("commands/wantflac");

        cargo_bin_cmd!("aur")
            .arg("--config")
            .arg(config_file_as_string())
            .arg("wantflac")
            .arg("--root")
            .arg(&dir_under_test)
            .assert()
            .success()
            .stdout(sample_output("commands/wantflac/wantflac.txt"));
    }

    #[test]
    #[ignore]
    fn test_wantflac_command_invalid_tree() {
        cargo_bin_cmd!("aur")
            .args(["wantflac", "--root", "/tmp"])
            .assert()
            .failure()
            .stderr("ERROR: did not find /tmp/mp3\n");
    }
}
