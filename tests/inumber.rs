mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_inumber_command() {
        let file_name = "13.change_both.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/inumber"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&[
                "inumber",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .stdin("1")
            .succeeds()
            .and()
            .stdout()
            .contains("13.change_both.mp3 > ")
            .and()
            .stdout()
            .contains("t_num -> 1")
            .and()
            .stdout()
            .contains("13.change_both.mp3 -> 01.change_both.mp3")
            .unwrap();

        // Nothing should happen this time, but we'll still get the prompt

        let new_file_under_test = tmp.path().join("01.change_both.mp3");

        assert_cli::Assert::main_binary()
            .with_args(&[
                "inumber",
                new_file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .stdin("1")
            .succeeds()
            .and()
            .stdout()
            .is("01.change_both.mp3 > ")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_inumber_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["inumber", "/no/such/file.flac"])
            .stdin("1")
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_inumber_bad_input() {
        assert_cli::Assert::main_binary()
            .with_args(&["inumber", "/no/such/file.flac"])
            .stdin("merp")
            .fails()
            .and()
            .stderr()
            .is("ERROR: (Parsing): invalid digit found in string")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_inumber_incorrect_usage() {
        common::missing_file_args_test("inumber");
    }
}