use crate::separator;
use crate::utils::dir;
use crate::utils::metadata::AurMetadata;
use camino::Utf8PathBuf;
use tabled::Table;
use tabled::settings::{Alignment, Modify, Remove, Style, object::Columns, object::Rows};

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<bool> {
    let mut ret = true;

    for file in &dir::media_files(&dir::pathbuf_set(files)) {
        separator!(file, files);

        if let Ok(mut metadata) = AurMetadata::new(file) {
            let tags_table = file_tags(&mut metadata);
            println!("{tags_table}");
        } else {
            eprintln!("Cannot get metadata");
            ret = false;
        }
    }

    Ok(ret)
}

fn file_tags(metadata: &mut AurMetadata) -> String {
    metadata.rawtags.sort();
    let table_rows = &metadata.rawtags;

    let mut table = Table::new(table_rows);
    let style = Style::blank().vertical(':');

    table
        .with(style)
        .with(Remove::row(Rows::first())) // headers
        .with(Modify::new(Columns::first()).with(Alignment::right()));

    table.to_string()
}
