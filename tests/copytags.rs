mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_copytags_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/copytags"), &["**/*"])
            .unwrap();
        let file_under_test = tmp.path().join("mp3").join("01.artist.song.mp3");
        let expected_output = "
           title -> Right Title
           album -> Copytags Test
           t_num -> 1
            date -> 2021";

        // assert_cli::Assert::command(&["find", "."])
        //     .current_dir(tmp.path())
        //     .succeeds()
        //     .and()
        //     .stdout()
        //     .is("merp")
        //     .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "get",
                "title",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("Wrong Title")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "copytags",
                "--force",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is(expected_output)
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "get",
                "title",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("Right Title")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "copytags",
                "--force",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    fn test_copytags_incorrect_usage() {
        common::missing_file_args_test("copytags");
    }
}
