mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[ignore]
    #[test]
    fn test_artfix_link() {
        let source = assert_fs::TempDir::new().unwrap();
        let linkdir = assert_fs::TempDir::new().unwrap();
        source
            .copy_from(fixture("commands/artfix"), &["tester.all_wrong/*"])
            .unwrap();
        let dir_under_test = source.join("tester.all_wrong");

        assert!(dir_under_test.join("some_file.JPEG").exists());
        assert!(!dir_under_test.join("front.jpg").exists());

        assert_cli::Assert::main_binary()
            .with_args(&[
                "artfix",
                "-d",
                &linkdir.to_string_lossy(),
                &dir_under_test.to_string_lossy(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains(
                format!(
                    "Rename: {dir}/some_file.JPEG -> front.jpg",
                    dir = dir_under_test.display()
                )
                .as_str(),
            )
            .unwrap();

        assert!(!dir_under_test.join("some_file.JPEG").exists());
        assert!(dir_under_test.join("front.jpg").exists());

        assert_cli::Assert::command(&["ls", "-l", &linkdir.to_string_lossy()])
            .stdout()
            .contains(
                format!(
                    "-tester.all_wrong-front.jpg -> {}/front.jpg",
                    dir_under_test.to_string_lossy()
                )
                .as_str(),
            )
            .unwrap()
    }

    #[ignore]
    #[test]
    fn test_artfix_scale() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/artfix"), &["tester.too_big/*"])
            .unwrap();
        let dir_under_test = tmp.join("tester.too_big");

        assert_cli::Assert::main_binary()
            .with_args(&["artfix", &dir_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .contains(format!("Resize: {}/front.jpg -> 750x750", dir_under_test.display()).as_str())
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["artfix", &dir_under_test.to_string_lossy()])
            .succeeds()
            .and()
            .stdout()
            .is("")
            .unwrap();
    }

    #[ignore]
    #[test]
    fn test_artfix_incorrect_usage() {
        common::missing_file_args_test("artfix");
    }
}
