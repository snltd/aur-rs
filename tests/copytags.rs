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

        // Check the title is what we think it is (and wrong)
        assert_cli::Assert::main_binary()
            .with_args(&["get", "title", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("Wrong Title")
            .unwrap();

        // Copy the tags
        assert_cli::Assert::main_binary()
            .with_args(&["copytags", "--force", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("title -> Right Title")
            .and()
            .stdout()
            .contains("album -> Copytags Test")
            .and()
            .stdout()
            .contains("t_num -> 1")
            .and()
            .stdout()
            .contains("year -> 2021")
            .unwrap();

        // Check the title is now correct
        assert_cli::Assert::main_binary()
            .with_args(&["get", "title", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("Right Title")
            .unwrap();

        // This time nothing should happen because the tags already match
        assert_cli::Assert::main_binary()
            .with_args(&["copytags", "--force", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    // I'm not sure at the moment whether I want this behaviour or not
    // #[test]
    // #[ignore]
    // fn test_copytags_command_no_partner() {
    //     let tmp = assert_fs::TempDir::new().unwrap();
    //     tmp.copy_from(fixture("commands/copytags"), &["**/*"])
    //         .unwrap();
    //     let file_under_test = tmp.path().join("mp3").join("02.artist.song.mp3");

    //     // Should fail because there's no corresponding FLAC
    //     assert_cli::Assert::main_binary()
    //         .with_args(&["copytags", "--force", &file_under_test.to_string_lossy()])
    //         .fails()
    //         .and()
    //         .stdout()
    //         .is("")
    //         .and()
    //         .stderr()
    //         .contains("has no partner from which to copy tags")
    //         .unwrap();
    // }

    #[test]
    #[ignore]
    fn test_copytags_incorrect_usage() {
        common::missing_file_args_test("copytags");
    }
}
