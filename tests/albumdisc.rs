#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    fn test_cli() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.child("album/disc_3").create_dir_all().unwrap();
        let target = tmp.child("album/disc_3");
        target
            .copy_from(
                fixture("commands/albumdisc/disc_3/"),
                &["01.artist.song.mp3"],
            )
            .unwrap();

        let file_under_test = target.path().join("01.artist.song.mp3");

        assert_cli::Assert::main_binary()
            .with_args(&[
                "albumdisc",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("album -> Test Album (Disc 3)")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "albumdisc",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }
}
