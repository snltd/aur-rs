mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_transcode_command_wav() {
        let file_name = "01.tester.lossless.wav";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/transcode"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();
        let expected_file = tmp.path().join("01.tester.lossless.flac");

        assert!(!expected_file.exists());

        assert_cli::Assert::main_binary()
            .with_args(&["transcode", "--verbose", "flac", &file_str])
            .stdout()
            .is(format!(
                "{} -> {}",
                file_under_test.display(),
                expected_file.display(),
            )
            .as_str())
            .unwrap();

        assert!(expected_file.exists());

        assert_cli::Assert::main_binary()
            .with_args(&["transcode", "--verbose", "flac", &file_str])
            .stdout()
            .is(format!(
                "target '{}' exists. Use -f to overwrite",
                expected_file.display()
            )
            .as_str())
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_transcode_incorrect_usage() {
        common::missing_file_args_test("transcode flac");
    }
}
