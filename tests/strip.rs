mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_strip_command_flac() {
        let file_name = "01.tester.not_stripped.flac";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/strip"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();

        assert_cli::Assert::main_binary()
            .with_args(&["lint", &file_str])
            .stdout()
            .contains("Unexpected tags: composer, tempo")
            .and()
            .stdout()
            .contains("File contains embedded artwork")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["strip", &file_str])
            .stdout()
            .contains(
                format!(
                    "Strip: {} :: composer, encoder, tempo",
                    file_under_test.display()
                )
                .as_str(),
            )
            .and()
            .stdout()
            .contains(format!("Strip: {} :: embedded artwork", file_under_test.display()).as_str())
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["lint", &file_str])
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_strip_command_mp3() {
        let file_name = "02.tester.not_stripped.mp3";
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/strip"), &[file_name])
            .unwrap();
        let file_under_test = tmp.path().join(file_name);
        let file_str = file_under_test.to_string_lossy();

        assert_cli::Assert::main_binary()
            .with_args(&["lint", &file_str])
            .stdout()
            .contains("Unexpected tags: apic, tcom, tenc, txxx")
            .and()
            .stdout()
            .contains("File contains embedded artwork")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["strip", &file_str])
            .stdout()
            .contains(
                format!(
                    "Strip: {} :: apic, tcom, tenc, tlen, tsse, txxx",
                    file_under_test.display()
                )
                .as_str(),
            )
            .and()
            .stdout()
            .contains(format!("Strip: {} :: embedded artwork", file_under_test.display()).as_str())
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["lint", &file_str])
            .stdout()
            .is("")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn test_strip_incorrect_usage() {
        common::missing_file_args_test("strip");
    }
}
