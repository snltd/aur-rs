use crate::err_if_empty;
use crate::utils::dir;
use crate::utils::rename;
use crate::utils::types::GlobalOpts;
use camino::Utf8PathBuf;

pub fn run(files: &[Utf8PathBuf], opts: &GlobalOpts) -> anyhow::Result<bool> {
    let mut ret_code = true;
    let files = dir::media_files(&dir::pathbuf_set(files));
    err_if_empty!(files);

    for file in files {
        match rename::rename_action_from_file(&file) {
            Ok(rename_opt) => {
                if let Some(action) = rename_opt
                    && let Err(e) = rename::rename(action, opts.noop)
                {
                    eprintln!("Error renaming {file}: {e}");
                    ret_code = false;
                }
            }
            Err(e) => {
                eprintln!("Error inspecting {file}: {e}");
                ret_code = false;
            }
        }
    }

    Ok(ret_code)
}
