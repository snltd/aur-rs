mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_dupes_command_valid_tree() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/dupes"), &["flac/**/*"])
            .unwrap();
        let file_under_test = tmp.path().join("flac");

        assert_cli::Assert::main_binary()
            .with_args(&["dupes", &file_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains("flac/tracks/fall.free_ranger.flac")
            .and()
            .stdout()
            .contains("flac/tracks/slint.don_aman.flac")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_dupes_command_invalid_tree() {
        assert_cli::Assert::main_binary()
            .with_args(&["dupes", "/tmp"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: /tmp/tracks not found")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_dupes_incorrect_usage() {
        common::missing_file_args_test("dupes");
    }
}
