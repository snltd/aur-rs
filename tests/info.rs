#[cfg(test)]
mod test {
    use aur::test_utils::spec_helper::{fixture_as_string, sample_output};

    #[test]
    #[ignore]
    fn test_info_command() {
        assert_cli::Assert::main_binary()
            .with_args(&[
                "info",
                fixture_as_string("commands/tags/01.test_artist.test_track.flac").as_str(),
            ])
            .stdout()
            .is(sample_output("commands/info/01.test_artist.test_track.flac.txt").as_str())
            .unwrap();
    }
}
