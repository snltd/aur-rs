use camino::{Utf8Path, Utf8PathBuf};
use std::collections::BTreeSet;

pub fn pathbuf_set(files: &[Utf8PathBuf]) -> BTreeSet<Utf8PathBuf> {
    files.iter().map(Utf8PathBuf::from).collect()
}

pub fn media_files<T>(flist: &T) -> T
where
    T: IntoIterator<Item = Utf8PathBuf> + FromIterator<Utf8PathBuf> + Clone,
{
    let flac_ext = "flac";
    let mp3_ext = "mp3";

    flist
        .clone()
        .into_iter()
        .filter(|f| match f.extension() {
            Some(ext) => ext == flac_ext || ext == mp3_ext,
            None => false,
        })
        .collect()
}

pub fn expand_file_list(
    flist: &[Utf8PathBuf],
    recurse: bool,
) -> anyhow::Result<BTreeSet<Utf8PathBuf>> {
    let mut ret: BTreeSet<Utf8PathBuf> = BTreeSet::new();
    let mut dirlist: Vec<Utf8PathBuf> = Vec::new();

    for f in flist.iter().cloned() {
        if f.is_file() {
            ret.insert(f);
        } else if f.is_dir() {
            dirlist.push(f);
        }
    }

    if recurse {
        for dir in expand_dir_list(&dirlist, true) {
            for entry in dir.read_dir_utf8()? {
                let path = entry?.into_path();
                if path.is_file() {
                    ret.insert(path);
                }
            }
        }
    }

    Ok(ret)
}

pub fn expand_dir_list(dirlist: &[Utf8PathBuf], recurse: bool) -> BTreeSet<Utf8PathBuf> {
    if recurse {
        dirs_under(dirlist)
    } else {
        dirlist.iter().map(Utf8PathBuf::from).collect()
    }
}

fn dirs_under(dirs: &[Utf8PathBuf]) -> BTreeSet<Utf8PathBuf> {
    let mut ret = BTreeSet::new();

    for dir in dirs {
        let path = Utf8Path::new(&dir);
        if path.is_dir() {
            collect_directories(path, &mut ret);
        }
    }

    ret.into_iter().collect()
}

fn collect_directories(dir: &Utf8Path, aggr: &mut BTreeSet<Utf8PathBuf>) {
    aggr.insert(dir.to_path_buf());

    if let Ok(entries) = dir.read_dir_utf8() {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                collect_directories(entry.path(), aggr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::spec_helper::fixture;
    use assert_unordered::assert_eq_unordered;
    use camino_tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_media_files() {
        let input = vec![
            Utf8PathBuf::from("/flac/ep/01.singer.song_01.flac"),
            Utf8PathBuf::from("/flac/ep/02.singer.song_02.flac"),
            Utf8PathBuf::from("/flac/ep/front.jpg"),
            Utf8PathBuf::from("/mp3/album/01.singer.song_01.mp3"),
            Utf8PathBuf::from("/mp3/album/02.singer.song_02.mp3"),
            Utf8PathBuf::from("/mp3/album/03.singer.song_03.mp3"),
            Utf8PathBuf::from("/mp3/album/something_that_should_not_be_there"),
        ];

        let expected = vec![
            Utf8PathBuf::from("/flac/ep/01.singer.song_01.flac"),
            Utf8PathBuf::from("/flac/ep/02.singer.song_02.flac"),
            Utf8PathBuf::from("/mp3/album/01.singer.song_01.mp3"),
            Utf8PathBuf::from("/mp3/album/02.singer.song_02.mp3"),
            Utf8PathBuf::from("/mp3/album/03.singer.song_03.mp3"),
        ];

        assert_eq_unordered!(expected, media_files(&input));
    }

    #[test]
    fn test_expand_file_list_no_recurse() {
        let result = expand_file_list(
            &[
                fixture("recurse/flac/tracks/band.single.flac"),
                fixture("recurse/flac/eps"),
            ],
            false,
        );

        let mut expected: BTreeSet<Utf8PathBuf> = BTreeSet::new();
        expected.insert(fixture("recurse/flac/tracks/band.single.flac"));
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_expand_file_list_recurse() {
        let expected: BTreeSet<Utf8PathBuf> = BTreeSet::from([
            fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_1/01.test_artist.disc_1--song_1.flac"),
            fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_1/02.test_artist.disc_1--song_2.flac"),
            fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_1/03.test_artist.disc_1--song_3.flac"),
            fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_2/03.test_artist.disc_2--song_3.flac"),
            fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_2/02.test_artist.disc_2--song_2.flac"),
            fixture("recurse/flac/albums/tuv/test_artist.test_album/disc_2/01.test_artist.disc_2--song_1.flac"),
            fixture("recurse/flac/eps/artist.extended_play/02.artist.ep_02.flac"),
            fixture("recurse/flac/tracks/band.single.flac"),
        ]);

        let result = expand_file_list(
            &[
                fixture("recurse/flac/tracks"),
                fixture("recurse/flac/eps/artist.extended_play/02.artist.ep_02.flac"),
                fixture("recurse/flac/albums/tuv/test_artist.test_album"),
            ],
            true,
        );

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

        let dirs = [temp_dir.path().into(), subdir3.as_path().into()];
        let all_dirs = dirs_under(&dirs);

        let expected_dirs: BTreeSet<_> =
            vec![temp_dir.path().to_path_buf(), subdir1, subdir2, subdir3]
                .into_iter()
                .collect();

        let result_dirs: BTreeSet<_> = all_dirs.into_iter().collect();
        assert_eq!(result_dirs, expected_dirs);
    }

    #[test]
    fn test_expand_dir_list_recurse_mp3() {
        let result = expand_dir_list(
            &[fixture("recurse/mp3/albums"), fixture("recurse/mp3/eps")],
            true,
        );

        let expected = BTreeSet::from([
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
        let result = expand_dir_list(&[fixture("recurse/albums"), fixture("recurse/eps")], false);

        let expected = BTreeSet::from([fixture("recurse/albums"), fixture("recurse/eps")]);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_expand_dir_list_recurse_flac() {
        let expected = BTreeSet::from([
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
            &[
                fixture("recurse/flac/eps"),
                fixture("recurse/flac/albums"),
                fixture("recurse/flac/tracks"),
            ],
            true,
        );

        assert_eq!(expected, result);
    }
}
