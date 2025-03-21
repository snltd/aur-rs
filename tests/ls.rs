#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use aur::test_utils::spec_helper::{fixture_as_string, sample_output};

    #[test]
    #[ignore]
    fn test_ls_command() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["ls", &fixture_as_string("commands/ls")])
            .assert()
            .success()
            .stdout(sample_output("commands/ls/ls.txt"));

        Command::cargo_bin("aur")
            .unwrap()
            .arg("ls")
            .current_dir(fixture_as_string("commands/ls"))
            .assert()
            .success()
            .stdout(sample_output("commands/ls/ls.txt"));
    }

    #[test]
    #[ignore]
    fn test_ls_command_no_media() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["ls", "/"])
            .assert()
            .success()
            .stdout("\n")
            .stderr("");
    }

    #[test]
    #[ignore]
    fn test_ls_command_no_dir() {
        Command::cargo_bin("aur")
            .unwrap()
            .args(["ls", "/no/such/directory"])
            .assert()
            .failure()
            .stdout("")
            .stderr("ERROR: (I/O) : No such file or directory (os error 2)\n");
    }
}
