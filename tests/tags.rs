mod common;

#[cfg(test)]
mod test {
    use super::common;
    use aur::test_utils::spec_helper::{fixture_as_string, sample_output};

    #[test]
    #[ignore]
    fn test_tags_command() {
        assert_cli::Assert::main_binary()
            .with_args(&[
                "tags",
                fixture_as_string("commands/tags/01.test_artist.test_track.flac").as_str(),
            ])
            .stdout()
            .is(sample_output("commands/tags/01.test_artist.test_track.flac.txt").as_str())
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_tags_incorrect_usage() {
        common::missing_file_args_test("tags");
    }
}
