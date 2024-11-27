mod common;

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};

    #[test]
    #[ignore]
    fn test_name2tag_command() {
        let file_name = "01.test_artist.untagged_song.flac";

        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.child("test.artist.test_album")
            .create_dir_all()
            .unwrap();
        let target = tmp.child("test_artist.test_album");
        target
            .copy_from(fixture("commands/name2tag"), &[file_name])
            .unwrap();

        let file_under_test = target.path().join(file_name);

        assert_cli::Assert::main_binary()
            .with_args(&[
                "name2tag",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("t_num -> 1")
            .and()
            .stdout()
            .contains("artist -> Test Artist")
            .and()
            .stdout()
            .contains("album -> Test Album")
            .and()
            .stdout()
            .contains("title -> Untagged Song")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "name2tag",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_name2tag_command_bad_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["name2tag", fixture_as_string("info/bad_file.flac").as_str()])
            .fails()
            .and()
            .stderr()
            .is("ERROR: InvalidInput: reader does not contain flac metadata")
            .unwrap();
    }

    // #[test]
    // #[ignore]
    // fn test_name2tag_command_missing_file() {
    //     assert_cli::Assert::main_binary()
    //         .with_args(&["name2tag", "/no/such/file.flac"])
    //         .fails()
    //         .and()
    //         .stderr()
    //         .is("ERROR: (I/O) : No such file or directory (os error 2)")
    //         .unwrap();
    // }

    // #[test]
    // #[ignore]
    // fn test_name2tag_incorrect_usage() {
    //     common::missing_file_args_test("name2tag");
    // }
}
