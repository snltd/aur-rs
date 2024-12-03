mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_cdq_command_overwrite() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["01.tester.hi-res.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.hi-res.flac");
        let cdq_file = tmp.path().join("01.tester.hi-res-cdq.flac");

        assert_cli::Assert::main_binary()
            .with_args(&["cdq", &file_under_test.to_string_lossy()])
            .succeeds()
            .unwrap();

        assert!(!cdq_file.exists());

        assert_cli::Assert::main_binary()
            .with_args(&["get", "bitrate", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("16-bit/44100Hz")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_cdq_command_leave_original() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["01.tester.hi-res.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.hi-res.flac");
        let cdq_file = tmp.path().join("01.tester.hi-res-cdq.flac");

        assert_cli::Assert::main_binary()
            .with_args(&["cdq", "-l", &file_under_test.to_string_lossy()])
            .succeeds()
            .unwrap();

        assert!(cdq_file.exists());

        assert_cli::Assert::main_binary()
            .with_args(&["get", "bitrate", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("24-bit/96000Hz")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "bitrate", &cdq_file.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("16-bit/44100Hz")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_cdq_command_mp3() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/cdq"), &["02.tester.not_a_flac.mp3"])
            .unwrap();
        let file_under_test = tmp.path().join("02.tester.not_a_flac.mp3");

        assert_cli::Assert::main_binary()
            .with_args(&["cdq", &file_under_test.to_string_lossy()])
            .fails()
            .and()
            .stderr()
            .is("ERROR: Only FLAC files can be CDQed")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_cdq_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["cdq", "up", "2", "/no/such/file.flac"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_cdq_incorrect_usage() {
        common::missing_file_args_test("cdq");
    }
}
