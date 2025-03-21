#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use aur::test_utils::spec_helper::{config_file_as_string, fixture_as_string, sample_output};

    #[test]
    #[ignore]
    fn test_wantflac_command_valid_tree() {
        let dir_under_test = fixture_as_string("commands/wantflac");

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "--config",
                &config_file_as_string(),
                "wantflac",
                "--root",
                &dir_under_test,
            ])
            .assert()
            .success()
            .stdout(sample_output("commands/wantflac/wantflac.txt"));
    }

    #[test]
    #[ignore]
    fn test_wantflac_command_invalid_tree() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["wantflac", "--root", "/tmp"])
            .assert()
            .failure()
            .stderr("ERROR: did not find /tmp/mp3\n");
    }
}
