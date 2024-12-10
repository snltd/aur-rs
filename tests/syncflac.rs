mod common;

#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::{fixture, sample_output};

    #[test]
    #[ignore]
    fn test_syncflac_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands"), &["syncflac/**/*"])
            .unwrap();
        let dir_under_test = tmp.path().join("syncflac");

        assert_cli::Assert::main_binary()
            .with_args(&[
                "syncflac",
                "--verbose",
                "-R",
                &dir_under_test.to_string_lossy(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("Creating directory")
            .and()
            .stdout()
            .contains("Transcoding")
            .and()
            .stdout()
            .contains("Removing")
            .unwrap();

        assert_cli::Assert::command(&["ls", "-R"])
            .current_dir(&dir_under_test)
            .stdout()
            .is(sample_output("commands/syncflac/ls").as_str())
            .unwrap();

        let sample_file = dir_under_test
            .join("mp3/albums/tuv/tester.flac_album")
            .join("01.tester.song_1.mp3");

        assert_cli::Assert::main_binary()
            .with_args(&["info", &sample_file.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is(sample_output("commands/syncflac/info-new-mp3").as_str())
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["syncflac", "-R", &dir_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_syncflac_bad_directory() {
        assert_cli::Assert::main_binary()
            .with_args(&["syncflac", "--root", "/tmp"])
            .fails()
            .and()
            .stdout()
            .is("")
            .and()
            .stderr()
            .is("ERROR: did not find /tmp/mp3")
            .unwrap();
    }
}
