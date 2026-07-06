#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use snltest::{fixture, load_fixture};

    #[test]
    #[ignore]
    fn test_ls_command() {
        cargo_bin_cmd!("aur")
            .arg("ls")
            .arg(fixture!("commands/ls"))
            .assert()
            .success()
            .stdout(load_fixture!("outputs/commands/ls/ls.txt"));
    }

    #[test]
    #[ignore]
    fn test_ls_command_no_media() {
        cargo_bin_cmd!("aur")
            .args(["ls", "/"])
            .assert()
            .success()
            .stdout("\n")
            .stderr("");
    }

    #[test]
    #[ignore]
    fn test_ls_command_no_dir() {
        cargo_bin_cmd!("aur")
            .args(["ls", "/no/such/directory"])
            .assert()
            .failure()
            .stdout("")
            .stderr("Error listing /no/such/directory: No such file or directory (os error 2)\n");
    }
}
