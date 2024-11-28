mod common;

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};

    #[test]
    #[ignore]
    fn test_retitle_command() {
        let file_name = "02.test_artist.this_title_needs_sorting.flac";

        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/retitle"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&["retitle", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("artist -> Test Artist")
            .and()
            .stdout()
            .contains("album -> Test the Retitle")
            .and()
            .stdout()
            .contains("title -> This Title Needs Sorting")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["retitle", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_retitle_command_bad_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["retitle", &fixture_as_string("info/bad_file.flac")])
            .fails()
            .and()
            .stderr()
            .is("ERROR: InvalidInput: reader does not contain flac metadata")
            .unwrap();
    }

    // #[test]
    // #[ignore]
    // fn test_retitle_command_missing_file() {
    //     assert_cli::Assert::main_binary()
    //         .with_args(&["retitle", "/no/such/file.flac"])
    //         .fails()
    //         .and()
    //         .stderr()
    //         .is("ERROR: (I/O) : No such file or directory (os error 2)")
    //         .unwrap();
    // }

    // #[test]
    // #[ignore]
    // fn test_retitle_incorrect_usage() {
    //     common::missing_file_args_test("retitle");
    // }
}
