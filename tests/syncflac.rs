#[cfg(test)]
mod test {
    use assert_cmd::cargo::cargo_bin_cmd;
    use aur::test_utils::spec_helper::{fixture, sample_output};
    use camino_tempfile_ext::prelude::*;
    use glob::glob;
    use predicates::prelude::*;
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    #[test]
    #[ignore]
    fn test_syncflac_command() {
        let tmp = Utf8TempDir::new().unwrap();
        tmp.copy_from(fixture("commands"), &["syncflac/**/*"])
            .unwrap();
        let dir_under_test = tmp.path().canonicalize().unwrap().join("syncflac");

        cargo_bin_cmd!("aur")
            .arg("syncflac")
            .arg("--verbose")
            .arg("-R")
            .arg(&dir_under_test)
            .assert()
            .success()
            .stdout(predicate::str::contains("Creating target"))
            .stdout(predicate::str::contains(format!(
                "Removing {}",
                tmp.path()
                    .canonicalize_utf8()
                    .unwrap()
                    .join("syncflac/mp3/eps/band.flac_and_mp3_unequal/03.band.song_3.mp3")
            )));

        let mut expected_files = BTreeSet::new();

        let files = [
            "flac",
            "flac/albums",
            "flac/albums/abc",
            "flac/albums/abc/already.synced",
            "flac/albums/abc/already.synced/01.already.synced.flac",
            "flac/albums/tuv",
            "flac/albums/tuv/tester.flac_album",
            "flac/albums/tuv/tester.flac_album/01.tester.song_1.flac",
            "flac/albums/tuv/tester.flac_album/02.tester.song_2.flac",
            "flac/eps",
            "flac/eps/band.flac_and_mp3_unequal",
            "flac/eps/band.flac_and_mp3_unequal/01.band.song_1.flac",
            "flac/eps/band.flac_and_mp3_unequal/02.band.song_2.flac",
            "flac/eps/band.flac_ep",
            "flac/eps/band.flac_ep/01.band.song_1.flac",
            "flac/eps/band.flac_ep/02.band.song_2.flac",
            "flac/tracks",
            "flac/tracks/singer.song.flac",
            "mp3",
            "mp3/albums",
            "mp3/albums/abc",
            "mp3/albums/abc/already.synced",
            "mp3/albums/abc/already.synced/01.already.synced.mp3",
            "mp3/albums/abc/artist.mp3_album",
            "mp3/albums/abc/artist.mp3_album/01.artist.song_1.mp3",
            "mp3/albums/abc/artist.mp3_album/02.artist.song_1.mp3",
            "mp3/albums/tuv",
            "mp3/albums/tuv/tester.flac_album",
            "mp3/albums/tuv/tester.flac_album/01.tester.song_1.mp3",
            "mp3/albums/tuv/tester.flac_album/02.tester.song_2.mp3",
            "mp3/eps",
            "mp3/eps/band.flac_and_mp3_unequal",
            "mp3/eps/band.flac_and_mp3_unequal/01.band.song_1.mp3",
            "mp3/eps/band.flac_and_mp3_unequal/02.band.song_2.mp3",
            "mp3/eps/band.flac_ep",
            "mp3/eps/band.flac_ep/01.band.song_1.mp3",
            "mp3/eps/band.flac_ep/02.band.song_2.mp3",
            "mp3/eps/band.mp3_ep",
            "mp3/eps/band.mp3_ep/01.band.song_1.mp3",
            "mp3/eps/band.mp3_ep/02.band.song_2.mp3",
            "mp3/tracks",
            "mp3/tracks/singer.song.mp3",
            "mp3/tracks/this.that.mp3",
        ];

        for f in files {
            expected_files.insert(dir_under_test.join(f));
        }

        let glob_pattern = format!("{}/**/*", dir_under_test.display());

        let actual_files: BTreeSet<PathBuf> = glob(glob_pattern.as_str())
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        assert_eq!(expected_files, actual_files);

        let sample_file = dir_under_test
            .join("mp3/albums/tuv/tester.flac_album")
            .join("01.tester.song_1.mp3");

        cargo_bin_cmd!("aur")
            .arg("info")
            .arg(&sample_file)
            .assert()
            .success()
            .stdout(sample_output("commands/syncflac/info-new-mp3"));

        cargo_bin_cmd!("aur")
            .arg("syncflac")
            .arg("-R")
            .arg(&dir_under_test)
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    #[ignore]
    fn test_syncflac_bad_directory() {
        cargo_bin_cmd!("aur")
            .args(["syncflac", "--root", "/usr"])
            .assert()
            .failure()
            .stdout("")
            .stderr("ERROR: did not find /usr/mp3\n");
    }
}
