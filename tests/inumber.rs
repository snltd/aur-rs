#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    fn test_cli() {
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
}
