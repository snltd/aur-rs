mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_thes_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/thes"), &["*"]).unwrap();
        let file_under_test = tmp.path().join("01.tester.song.mp3");

        assert_cli::Assert::main_binary()
            .with_args(&["thes", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("artist -> The Tester")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["thes", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_thes_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["thes", "/no/such/file.flac"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_thes_incorrect_usage() {
        common::missing_file_args_test("thes");
    }
}
