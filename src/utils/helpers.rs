use anyhow::anyhow;
use std::path::Path;

pub fn check_hierarchy(root: &Path) -> anyhow::Result<()> {
    let mp3_root = root.join("mp3");
    let flac_root = root.join("flac");

    if !mp3_root.exists() {
        return Err(anyhow!(format!("did not find {}", mp3_root.display())));
    }

    if !flac_root.exists() {
        return Err(anyhow!(format!("did not find {}", flac_root.display())));
    }

    Ok(())
}
