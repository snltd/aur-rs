#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use aur::test_utils::spec_helper::fixture;
    use predicates::prelude::*;

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

        Command::cargo_bin("aur")
            .unwrap()
            .args([
                "sort",
                &dir_under_test
                    .join("01 Some People Name Stuff Like.This!.flac")
                    .to_string_lossy(),
                &dir_under_test.join("01.singer.song.flac").to_string_lossy(),
                &dir_under_test
                    .join("Weird (\"Title\") by Singer.flac")
                    .to_string_lossy(),
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Creating {}",
                singers_album.display()
            )))
            .stdout(predicate::str::contains(
                "Weird (\"Title\") by Singer.flac -> singer.singers_album",
            ))
            .stdout(predicate::str::contains(format!(
                "Creating {}",
                test_album.display()
            )))
            .stdout(predicate::str::contains(
                "01 Some People Name Stuff Like.This!.flac -> test_artist.test_album",
            ))
            .stdout(predicate::str::contains(
                "01.singer.song.flac -> singer.singers_album",
            ));

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
        Command::cargo_bin("aur")
            .unwrap()
            .arg("sort")
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "the following required arguments were not provided",
            ));
    }
}
