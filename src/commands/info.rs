use crate::utils::dir;
use crate::utils::metadata::AurMetadata;
use crate::{err_if_empty, separator};
use camino::Utf8PathBuf;

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<bool> {
    let files = dir::media_files(&dir::pathbuf_set(files));
    err_if_empty!(files);

    let mut ret_code = true;

    for file in &files {
        separator!(file, files);

        match AurMetadata::new(file) {
            Ok(metadata) => {
                let info_table = file_info(&metadata);
                println!("{info_table}");
            }
            Err(e) => {
                eprintln!("Error getting metadata for {file}: {e}");
                ret_code = false;
            }
        }
    }

    Ok(ret_code)
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

    table_rows
        .iter()
        .map(|(k, v)| format!("{k:>10} : {v}\n"))
        .collect()
}
