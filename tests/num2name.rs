mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};

    #[test]
    #[ignore]
    fn test_num2name_command() {
        let file_name = "01.test_artist.test_title.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/num2name"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&[
                "num2name",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("01.test_artist.test_title.flac -> 02.test_artist.test_title.flac")
            .unwrap();

        let renamed_file = tmp.path().join("02.test_artist.test_title.flac");

        assert_cli::Assert::main_binary()
            .with_args(&[
                "num2name",
                renamed_file.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_num2name_command_bad_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["num2name", fixture_as_string("info/bad_file.flac").as_str()])
            .fails()
            .and()
            .stderr()
            .is("ERROR: InvalidInput: reader does not contain flac metadata")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_num2name_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["num2name", "/no/such/file.flac"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_num2name_incorrect_usage() {
        common::missing_file_args_test("num2name");
    }
}
