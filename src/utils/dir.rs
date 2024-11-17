use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn expand_dirlist(dirlist: Vec<String>, recurse: bool) -> Vec<PathBuf> {
    if recurse {
        dirs_under(dirlist)
    } else {
        dirlist.iter().map(PathBuf::from).collect()
    }
}

fn dirs_under(dirs: Vec<String>) -> Vec<PathBuf> {
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
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_dirs_under() {
        let temp_dir = tempdir().unwrap();
        let subdir1 = temp_dir.path().join("subdir1");
        let subdir2 = temp_dir.path().join("subdir1/subdir2");
        let subdir3 = temp_dir.path().join("subdir3");

        fs::create_dir_all(&subdir1).unwrap();
        fs::create_dir_all(&subdir2).unwrap();
        fs::create_dir_all(&subdir3).unwrap();

        let dirs = vec![
            temp_dir.path().to_string_lossy().to_string(),
            subdir3.to_string_lossy().to_string(),
        ];

        let all_dirs = dirs_under(dirs);

        let expected_dirs: HashSet<_> =
            vec![temp_dir.path().to_path_buf(), subdir1, subdir2, subdir3]
                .into_iter()
                .collect();

        let result_dirs: HashSet<_> = all_dirs.into_iter().collect();
        assert_eq!(result_dirs, expected_dirs);
    }
}
