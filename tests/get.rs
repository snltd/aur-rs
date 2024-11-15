#[cfg(test)]
mod test {
    use aur::test_utils::spec_helper::fixture_as_string;

    #[test]
    fn test_cli() {
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
            .with_args(&["get", "whatever", fixture.as_str()])
            .fails()
            // .and()
            // .stderr()
            // .is("ERROR: Unknown property")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["get", "title", "/no/such/file.flac"])
            .fails()
            // .and()
            // .stderr()
            // .is("ERROR: (I/O) : No such file or directory (os error 2)")
            .unwrap();
    }
}
