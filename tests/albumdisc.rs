mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    const FILENAME: &str = "01.artist.song.mp3";

    #[ignore]
    #[test]
    fn test_albumdisc_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.child("album/disc_3").create_dir_all().unwrap();
        let target = tmp.child("album/disc_3");
        target
            .copy_from(fixture("commands/albumdisc/disc_3/"), &[FILENAME])
            .unwrap();

        let file_under_test = target.path().join(FILENAME);

        assert_cli::Assert::main_binary()
            .with_args(&["albumdisc", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("album -> Test Album (Disc 3)")
            .unwrap();

        // Running again should do nothing, because it's been corrected
        assert_cli::Assert::main_binary()
            .with_args(&["albumdisc", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[ignore]
    #[test]
    fn test_albumdisc_file_not_in_disc_directory() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/albumdisc/disc_3"), &[FILENAME])
            .unwrap();
        let file_under_test = tmp.path().join(FILENAME);

        assert_cli::Assert::main_binary()
            .with_args(&["albumdisc", &file_under_test.to_string_lossy()])
            .fails()
            .and()
            .stderr()
            .contains("is not in a disc_n directory")
            .unwrap();
    }

    #[ignore]
    #[test]
    fn test_albumdisc_incorrect_usage() {
        common::missing_file_args_test("albumdisc");
    }
}
