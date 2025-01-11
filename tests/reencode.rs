mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use std::fs;

    #[test]
    #[ignore]
    fn test_reencode_command_flac_keep_original() {
        let file_name = "01.tester.song.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/reencode"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();
        let expected_file = tmp.path().join("01.tester.song.reencoded.flac");

        assert!(!expected_file.exists());

        assert_cli::Assert::main_binary()
            .with_args(&["reencode", "-k", &file_str])
            .stdout()
            .is(file_under_test.display().to_string().as_str())
            .unwrap();

        assert!(file_under_test.exists());
        assert!(expected_file.exists());
    }

    #[test]
    #[ignore]
    fn test_reencode_command_mp3_overwrite_original() {
        let file_name = "02.tester.song.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/reencode"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();
        let intermediate_file = tmp.path().join("02.tester.song.reencoded.mp3");

        assert!(!intermediate_file.exists());
        let original_size = fs::metadata(&file_under_test).unwrap().len();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "-s", "bitrate", &file_str])
            .stdout()
            .is("320kbps")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["reencode", &file_str])
            .stdout()
            .is(file_under_test.display().to_string().as_str())
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "-s", "bitrate", &file_str])
            .stdout()
            .is("128kbps")
            .unwrap();

        let new_size = fs::metadata(&file_under_test).unwrap().len();

        assert!(file_under_test.exists());
        assert!(!intermediate_file.exists());
        assert!(original_size > new_size);
    }

    #[test]
    #[ignore]
    fn test_reencode_incorrect_usage() {
        common::missing_file_args_test("reencode");
    }
}
