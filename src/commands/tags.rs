use crate::utils::dir;
use crate::utils::metadata::AurMetadata;
use crate::{err_if_empty, separator};
use camino::Utf8PathBuf;

pub fn run(files: &[Utf8PathBuf]) -> anyhow::Result<bool> {
    let mut ret_code = true;
    let files = &dir::media_files(&dir::pathbuf_set(files));
    err_if_empty!(files);

    for file in files {
        separator!(file, files);

        match AurMetadata::new(file) {
            Ok(mut metadata) => {
                let tags_table = file_tags(&mut metadata);
                println!("{tags_table}");
            }
            Err(e) => {
                eprintln!("Error getting metadata for {file}: {e}");
                ret_code = false;
            }
        }
    }

    Ok(ret_code)
}

fn file_tags(metadata: &mut AurMetadata) -> String {
    metadata.rawtags.sort();
    let table_rows = &metadata.rawtags;

    if let Some(longest_key_length) = table_rows.iter().map(|(k, _)| k.len()).max() {
        let key_length = if longest_key_length > 10 {
            longest_key_length + 1
        } else {
            10
        };

        table_rows
            .iter()
            .map(|(k, v)| format!("{:>width$} : {}\n", k, v, width = key_length))
            .collect()
    } else {
        String::new()
    }
}
