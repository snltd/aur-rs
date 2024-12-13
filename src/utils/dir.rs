use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

pub fn pathbuf_set(files: &[String]) -> HashSet<PathBuf> {
    files.iter().map(PathBuf::from).collect()
}

pub fn media_files<T>(flist: &T) -> T
where
    T: IntoIterator<Item = PathBuf> + FromIterator<PathBuf> + Clone,
{
    let flac_ext = OsString::from("flac");
    let mp3_ext = OsString::from("mp3");

    flist
        .clone()
        .into_iter()
        .filter(|f| match f.extension() {
            Some(ext) => ext == flac_ext || ext == mp3_ext,
            None => false,
        })
        .collect()
}

pub fn expand_file_list(flist: &[String], recurse: bool) -> anyhow::Result<HashSet<PathBuf>> {
    let mut ret: HashSet<PathBuf> = HashSet::new();
    let mut dirlist: Vec<PathBuf> = Vec::new();

    for file in flist {
        let f = PathBuf::from(&file);
        if f.is_file() {
            ret.insert(f);
        } else if f.is_dir() {
            dirlist.push(PathBuf::from(file));
        }
    }

    if recurse {
        for dir in expand_dir_list(&dirlist, true) {
            for entry in fs::read_dir(dir)? {
                let path = entry?.path();
                if path.is_file() {
                    ret.insert(path);
                }
            }
        }
    }

    Ok(ret)
}

pub fn expand_dir_list(dirlist: &[PathBuf], recurse: bool) -> HashSet<PathBuf> {
    if recurse {
        dirs_under(dirlist)
    } else {
        dirlist.iter().map(PathBuf::from).collect()
    }
}

fn dirs_under(dirs: &[PathBuf]) -> HashSet<PathBuf> {
    let mut ret = HashSet::new();

    for dir in dirs {
        let path = Path::new(&dir);
        if path.is_dir() {
            collect_directories(path, &mut ret);
        }
    }

    ret.into_iter().collect()
}

fn collect_directories(dir: &Path, aggr: &mut HashSet<PathBuf>) {
    aggr.insert(dir.to_path_buf());

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                collect_directories(&entry.path(), aggr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_unordered::assert_eq_unordered;
    use aur::test_utils::spec_helper::{fixture, fixture_as_string};
    use tempfile::tempdir;

    #[test]
    fn test_media_files() {
        let input = vec![
            PathBuf::from("/flac/ep/01.singer.song_01.flac"),
            PathBuf::from("/flac/ep/02.singer.song_02.flac"),
            PathBuf::from("/flac/ep/front.jpg"),
            PathBuf::from("/mp3/album/01.singer.song_01.mp3"),
            PathBuf::from("/mp3/album/02.singer.song_02.mp3"),
            PathBuf::from("/mp3/album/03.singer.song_03.mp3"),
            PathBuf::from("/mp3/album/something_that_should_not_be_there"),
        ];

        let expected = vec![
            PathBuf::from("/flac/ep/01.singer.song_01.flac"),
            PathBuf::from("/flac/ep/02.singer.song_02.flac"),
            PathBuf::from("/mp3/album/01.singer.song_01.mp3"),
            PathBuf::from("/mp3/album/02.singer.song_02.mp3"),
            PathBuf::from("/mp3/album/03.singer.song_03.mp3"),
        ];

        assert_eq_unordered!(expected, media_files(&input));
    }

    #[test]
    fn test_expand_file_list_no_recurse() {
        let result = expand_file_list(
            &[
                fixture_as_string("recurse/flac/tracks/band.single.flac"),
                fixture_as_string("recurse/flac/eps"),
            ],
            false,
        );

        let mut expected: HashSet<PathBuf> = HashSet::new();
        expected.insert(fixture("recurse/flac/tracks/band.single.flac"));
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_expand_file_list_recurse() {
        let result = expand_file_list(
            &[
                fixture_as_string("recurse/flac/tracks"),
                fixture_as_string("recurse/flac/eps/artist.extended_play/02.artist.ep_02.flac"),
                fixture_as_string("recurse/flac/albums/tuv/test_artist.test_album"),
            ],
            true,
        );

        let mut expected: HashSet<PathBuf> = HashSet::new();

        expected.insert(fixture("recurse/flac/tracks/band.single.flac"));
        expected.insert(fixture(
            "recurse/flac/eps/artist.extended_play/02.artist.ep_02.flac",
        ));
        expected.insert(fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_1/01.test_artist.disc_1--song_1.flac"),);
        expected.insert(fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_1/02.test_artist.disc_1--song_2.flac"),);
        expected.insert(fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_1/03.test_artist.disc_1--song_3.flac"),);
        expected.insert(fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_2/03.test_artist.disc_2--song_3.flac"),);
        expected.insert(fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_2/02.test_artist.disc_2--song_2.flac"),);
        expected.insert(fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_2/01.test_artist.disc_2--song_1.flac"),);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_dirs_under() {
        let temp_dir = tempdir().unwrap();
        let subdir1 = temp_dir.path().join("subdir1");
        let subdir2 = temp_dir.path().join("subdir1/subdir2");
        let subdir3 = temp_dir.path().join("subdir3");

        fs::create_dir_all(&subdir1).unwrap();
        fs::create_dir_all(&subdir2).unwrap();
        fs::create_dir_all(&subdir3).unwrap();

        let dirs = vec![temp_dir.path().to_path_buf(), subdir3.to_path_buf()];

        let all_dirs = dirs_under(&dirs);

        let expected_dirs: HashSet<_> =
            vec![temp_dir.path().to_path_buf(), subdir1, subdir2, subdir3]
                .into_iter()
                .collect();

        let result_dirs: HashSet<_> = all_dirs.into_iter().collect();
        assert_eq!(result_dirs, expected_dirs);
    }

    #[test]
    fn test_expand_dir_list_recurse_mp3() {
        let result = expand_dir_list(
            &vec![fixture("recurse/mp3/albums"), fixture("recurse/mp3/eps")],
            true,
        );

        let expected = HashSet::from([
            fixture("recurse/mp3/albums"),
            fixture("recurse/mp3/albums/abc"),
            fixture("recurse/mp3/albums/abc/artist.lp"),
            fixture("recurse/mp3/eps"),
            fixture("recurse/mp3/eps/artist.extended_play"),
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_expand_dir_list_no_recurse() {
        let result = expand_dir_list(
            &vec![fixture("recurse/albums"), fixture("recurse/eps")],
            false,
        );

        let expected = HashSet::from([fixture("recurse/albums"), fixture("recurse/eps")]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_expand_dir_list_recurse_flac() {
        let expected = HashSet::from([
            fixture("recurse/flac/albums"),
            fixture("recurse/flac/albums/pqrs"),
            fixture("recurse/flac/albums/pqrs/singer.album"),
            fixture("recurse/flac/albums/tuv"),
            fixture("recurse/flac/albums/tuv/test_artist.test_album"),
            fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_1"),
            fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_2"),
            fixture("recurse/flac/eps"),
            fixture("recurse/flac/eps/artist.extended_play"),
            fixture("recurse/flac/tracks"),
        ]);

        let result = expand_dir_list(
            &vec![
                fixture("recurse/flac/eps"),
                fixture("recurse/flac/albums"),
                fixture("recurse/flac/tracks"),
            ],
            true,
        );

        assert_eq!(expected, result);
    }
}
