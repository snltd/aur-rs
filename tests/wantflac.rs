#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use snltest::{fixture, load_fixture};

    #[test]
    #[ignore]
    fn test_wantflac_command_valid_tree() {
        let dir_under_test = fixture!("commands/wantflac");

        cargo_bin_cmd!("aur")
            .arg("--config")
            .arg(fixture!("config/test.toml"))
            .arg("wantflac")
            .arg("--root")
            .arg(&dir_under_test)
            .assert()
            .success()
            .stdout(load_fixture!("outputs/commands/wantflac/wantflac.txt"));
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
