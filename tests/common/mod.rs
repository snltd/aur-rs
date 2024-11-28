/// Used by the functional tests
#[allow(dead_code)]
pub fn missing_file_args_test(command: &str) {
    assert_cli::Assert::main_binary()
        .with_args(&[command])
        .fails()
        .and()
        .stderr()
        .contains("error: the following required arguments were not provided")
        .unwrap();
}
