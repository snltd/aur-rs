use crate::utils::metadata::{AurMetadata, AurTags};
use crate::utils::{dir, layout};
use crate::{err_if_empty, separator};
use camino::{Utf8Path, Utf8PathBuf};

pub fn run(dirlist: &[Utf8PathBuf], recurse: bool, long: bool) -> anyhow::Result<bool> {
    // If no argument is given, default to the cwd, just like real ls(1).
    let dirlist = if dirlist.is_empty() {
        &[Utf8PathBuf::from_path_buf(std::env::current_dir()?).unwrap()]
    } else {
        dirlist
    };

    let mut ret_code = true;
    let dirs = dir::expand_dir_list(dirlist, recurse);
    err_if_empty!(dirs);

    for dir in &dirs {
        separator!(dir, dirs);

        match list_info(dir) {
            Ok(_) => {
                let info = list_info(dir)?;
                let prepped_info = prep_table(info);

                if long {
                    for item in prepped_info {
                        println!("{}", item.join(" | "))
                    }
                } else {
                    println!("{}", layout::table(prepped_info))
                }
            }
            Err(e) => {
                eprintln!("Error listing {dir}: {e}");
                ret_code = false;
            }
        }
    }

    Ok(ret_code)
}

fn list_info(dir: &Utf8Path) -> anyhow::Result<Vec<AurTags>> {
    let entries = dir.read_dir_utf8()?;

    let mut all_file_tags: Vec<AurTags> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let file = entry.path();
            AurMetadata::new(file).ok().map(|metadata| metadata.tags)
        })
        .collect();

    all_file_tags.sort_by_key(|tags| tags.t_num);
    Ok(all_file_tags)
}

fn prep_table(items: Vec<AurTags>) -> Vec<Vec<String>> {
    items
        .into_iter()
        .map(|item| {
            vec![
                format!("{:02}", item.t_num),
                item.artist,
                item.title,
                item.album,
            ]
        })
        .collect()
}
