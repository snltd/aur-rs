#[cfg(test)]
mod test {
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    fn test_cli() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/dupes"), &["flac/**/*"])
            .unwrap();
        let file_under_test = tmp.path().join("flac");

        assert_cli::Assert::main_binary()
            .with_args(&[
                "dupes",
                file_under_test.to_string_lossy().to_string().as_str(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains("flac/tracks/fall.free_ranger.flac")
            .and()
            .stdout()
            .contains("flac/tracks/slint.don_aman.flac")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["dupes", "/tmp"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: /tmp/tracks not found")
            .unwrap();
    }
}
