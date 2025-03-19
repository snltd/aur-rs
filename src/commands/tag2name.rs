use crate::utils::dir;
use crate::utils::rename;
use crate::utils::types::GlobalOpts;
use camino::Utf8PathBuf;

pub fn run(files: &[Utf8PathBuf], opts: &GlobalOpts) -> anyhow::Result<()> {
    for f in dir::media_files(&dir::pathbuf_set(files)) {
        if let Some(action) = rename::rename_action_from_file(&f)? {
            rename::rename(action, opts.noop)?;
        }
    }

    Ok(())
}
