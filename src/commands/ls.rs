use crate::utils::dir;
use crate::utils::metadata::{AurMetadata, AurTags};
use crate::{err_if_empty, separator};
use camino::{Utf8Path, Utf8PathBuf};
use tabled::Table;
use tabled::settings::{
    Alignment, Modify, Padding, Remove, Style, Width, object::Columns, object::Rows,
    object::Segment,
};
use terminal_size::{Width as TermWidth, terminal_size};

pub fn run(dirlist: &[Utf8PathBuf], recurse: bool) -> anyhow::Result<bool> {
    // If no argument is given, default to the cwd, just like real ls(1).
    let dirlist = if dirlist.is_empty() {
        &[Utf8PathBuf::from_path_buf(std::env::current_dir()?).unwrap()]
    } else {
        dirlist
    };

    let mut ret_code = true;
    let dirs = dir::expand_dir_list(dirlist, recurse);
    err_if_empty!(dirs);

    let term_width: usize = terminal_size()
        .map(|(TermWidth(w), _)| w as usize)
        .unwrap_or(80);

    for dir in &dirs {
        separator!(dir, dirs);

        match list_info(dir) {
            Ok(_) => {
                let info = list_info(dir)?;
                println!("{}", listing_table(info, term_width));
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

fn listing_table(items: Vec<AurTags>, term_width: usize) -> String {
    let table_rows = items
        .into_iter()
        .map(|item| {
            (
                format!("{:02}", item.t_num),
                item.artist,
                item.title,
                item.album,
            )
        })
        .collect::<Vec<_>>();

    let mut table = Table::new(table_rows);

    table
        .with(Style::blank())
        .with(Remove::row(Rows::first())) // headers
        .with(Padding::new(0, 2, 0, 0))
        .with(Modify::new(Segment::all()).with(Width::wrap(term_width / 3).keep_words(true)))
        .with(Modify::new(Columns::last()).with(Alignment::right()));

    table.to_string()
}
