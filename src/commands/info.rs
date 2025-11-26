use crate::separator;
use crate::utils::dir::{media_files, pathbuf_set};
use crate::utils::metadata::AurMetadata;
use camino::Utf8PathBuf;
use tabled::Table;
use tabled::settings::{Alignment, Modify, Remove, Style, object::Columns, object::Rows};

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<bool> {
    let files = media_files(&pathbuf_set(files));
    let mut ret = true;

    for file in &files {
        separator!(file, files);

        if let Ok(metadata) = AurMetadata::new(file) {
            let info_table = file_info(&metadata);
            println!("{info_table}");
        } else {
            eprintln!("Cannot get metadata");
            ret = false;
        }
    }

    Ok(ret)
}

fn file_info(metadata: &AurMetadata) -> String {
    let table_rows = vec![
        ("Filename", metadata.filename.clone()),
        ("Type", metadata.filetype.to_uppercase()),
        ("Bitrate", metadata.quality().formatted),
        ("Time", metadata.time().formatted),
        ("Artist", metadata.tags.artist.clone()),
        ("Album", metadata.tags.album.clone()),
        ("Title", metadata.tags.title.clone()),
        ("Genre", metadata.tags.genre.clone()),
        ("Track no", metadata.tags.t_num.to_string()),
        ("Year", metadata.tags.year.to_string()),
    ];

    let mut table = Table::new(table_rows);
    let style = Style::blank().vertical(':');

    table
        .with(style)
        .with(Remove::row(Rows::first())) // headers
        .with(Modify::new(Columns::first()).with(Alignment::right()));

    table.to_string()
}
