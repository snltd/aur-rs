mod common;

#[cfg(test)]
mod test {
    use super::common;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;

    #[test]
    #[ignore]
    fn test_sort_command() {
        let tmp = assert_fs::TempDir::new().unwrap();
        tmp.copy_from(fixture("commands/sort"), &["**/*"]).unwrap();
        let dir_under_test = tmp.path();
        let singers_album = tmp.join("singer.singers_album");
        let test_album = tmp.join("test_artist.test_album");

        assert!(!singers_album.exists());
        assert!(!test_album.exists());

        assert_cli::Assert::main_binary()
            .with_args(&[
                "sort",
                &dir_under_test
                    .join("01 Some People Name Stuff Like.This!.flac")
                    .to_string_lossy(),
                &dir_under_test.join("01.singer.song.flac").to_string_lossy(),
                &dir_under_test
                    .join("Weird (\"Title\") by Singer.flac")
                    .to_string_lossy(),
            ])
            .succeeds()
            .and()
            .stdout()
            .contains(format!("Creating {}", singers_album.display()).as_str())
            .and()
            .stdout()
            .contains("Weird (\"Title\") by Singer.flac -> singer.singers_album")
            .and()
            .stdout()
            .contains(format!("Creating {}", test_album.display()).as_str())
            .and()
            .stdout()
            .contains("01 Some People Name Stuff Like.This!.flac -> test_artist.test_album")
            .and()
            .stdout()
            .contains("01.singer.song.flac -> singer.singers_album")
            .unwrap();

        assert!(singers_album.exists());
        assert!(test_album.exists());
        assert!(singers_album.join("01.singer.song.flac").exists());
        assert!(singers_album
            .join("Weird (\"Title\") by Singer.flac")
            .exists());
        assert!(test_album
            .join("01 Some People Name Stuff Like.This!.flac")
            .exists());
    }

    #[test]
    #[ignore]
    fn test_sort_incorrect_usage() {
        common::missing_file_args_test("sort");
    }
}
