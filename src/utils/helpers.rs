use anyhow::ensure;
use camino::Utf8Path;

pub fn check_hierarchy(root: &Utf8Path) -> anyhow::Result<()> {
    let mp3_root = root.join("mp3");
    let flac_root = root.join("flac");

    ensure!(mp3_root.exists(), format!("did not find {}", mp3_root));
    ensure!(flac_root.exists(), format!("did not find {}", flac_root));

    Ok(())
}
