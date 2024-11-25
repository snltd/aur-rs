mod common;

#[cfg(test)]
mod test {
    use super::common;
    use aur::test_utils::spec_helper::fixture_as_string;

    #[test]
    #[ignore]
    fn test_get_command_valid_property() {
        let fixture = fixture_as_string("commands/tags/01.test_artist.test_track.flac");

        // assert_cli appears to trim whitespace
        assert_cli::Assert::main_binary()
            .with_args(&["get", "title", fixture.as_str()])
            .succeeds()
            .and()
            .stdout()
            .is(format!("Test Track : {}", fixture).as_str())
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "bitrate", fixture.as_str()])
            .succeeds()
            .and()
            .stdout()
            .is(format!("16-bit/44100Hz : {}", fixture).as_str())
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_get_command_invalid_property() {
        let fixture = fixture_as_string("commands/tags/01.test_artist.test_track.flac");

        assert_cli::Assert::main_binary()
            .with_args(&["get", "whatever", fixture.as_str()])
            .fails()
            .and()
            .stderr()
            .is("ERROR: Unknown tag: whatever")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_get_command_missing_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["get", "title", "/no/such/file.flac"])
            .fails()
            .and()
            .stderr()
            .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_get_incorrect_usage() {
        common::missing_file_args_test("get");
    }
}
