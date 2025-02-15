mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, sample_output};
    use colored::Colorize;

    #[test]
    #[ignore]
    fn test_flac2mp3_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/flac2mp3"), &["01.tester.flac2mp3.flac"])
            .unwrap();
        let file_under_test = tmp.path().join("01.tester.flac2mp3.flac");
        let expected_file = tmp.path().join("01.tester.flac2mp3.mp3");

        assert!(!expected_file.exists());

        assert_cli::Assert::main_binary()
            .with_args(&["flac2mp3", &file_under_test.to_string_lossy()])
            .succeeds()
            .stdout()
            .is(format!(
                "{}\n  {}",
                file_under_test.display().to_string().bold(),
                file_under_test.file_name().unwrap().to_string_lossy(),
            )
            .as_str())
            .unwrap();

        assert!(file_under_test.exists());
        assert!(expected_file.exists());

        assert_cli::Assert::main_binary()
            .with_args(&["info", &expected_file.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is(sample_output("commands/flac2mp3/transcoded_info").as_str())
            .unwrap();

        // Probably should exit 1
        assert_cli::Assert::main_binary()
            .with_args(&["flac2mp3", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is(format!(
                "{}\n  target exists ({})",
                file_under_test.display().to_string().bold(),
                expected_file.display(),
            )
            .as_str())
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_command_mp3() {
        let file_under_test = fixture("commands/flac2mp3/01.tester.test_no-op.mp3");

        assert_cli::Assert::main_binary()
            .with_args(&["flac2mp3", &file_under_test.to_string_lossy()])
            .succeeds() //FIXME
            .and()
            .stderr()
            .is("ERROR: Only FLAC files can be flac2mp3-ed")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["flac2mp3", "/no/such/file.flac"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_flac2mp3_incorrect_usage() {
        common::missing_file_args_test("flac2mp3");
    }
}
