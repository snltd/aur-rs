#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    // #[ignore]
    fn test_renumber_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/renumber"), &["*"]).unwrap();
        let file_01_step_1 = tmp.path().join("01.test.song.flac");
        let file_02_step_1 = tmp.path().join("02.test.song.mp3");

        let file_01_step_2 = tmp.path().join("15.test.song.flac");
        let file_02_step_2 = tmp.path().join("16.test.song.mp3");

        let file_01_step_3 = tmp.path().join("08.test.song.flac");
        let file_02_step_3 = tmp.path().join("09.test.song.mp3");

        // Renumber upwards

        assert_cli::Assert::main_binary()
            .with_args(&[
                "renumber",
                "up",
                "14",
                file_01_step_1.to_string_lossy().to_string().as_str(),
                file_02_step_1.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("01.test.song.flac -> 15.test.song.flac")
            .and()
            .stdout()
            .contains("t_num -> 15")
            .and()
            .stdout()
            .contains("t_num -> 16")
            .and()
            .stdout()
            .contains("02.test.song.mp3 -> 16.test.song.mp3")
            .unwrap();

        // Renumber down

        assert_cli::Assert::main_binary()
            .with_args(&[
                "renumber",
                "down",
                "7",
                file_01_step_2.to_string_lossy().to_string().as_str(),
                file_02_step_2.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("15.test.song.flac -> 08.test.song.flac")
            .and()
            .stdout()
            .contains("t_num -> 8")
            .and()
            .stdout()
            .contains("t_num -> 9")
            .and()
            .stdout()
            .contains("16.test.song.mp3 -> 09.test.song.mp3")
            .unwrap();
    }

    // #[test]
    // fn test_renumber_command_bad_input() {
    //     let tmp = assert_fs::TempDir::new().unwrap();
    //     tmp.copy_from(fixture("commands/renumber"), &["02.test.song.mp3"])
    //         .unwrap();
    //     let file_under_test = tmp.path().join("02.test.song.mp3");

    //     assert_cli::Assert::main_binary()
    //         .with_args(&[
    //             "renumber",
    //             file_under_test.to_string_lossy().to_string().as_str(),
    //         ])
    //         .fails()
    //         .and()
    //         .stderr()
    //         .is("merp")
    //         .unwrap();
    // }
}
