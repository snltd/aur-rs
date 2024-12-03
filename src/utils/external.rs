use anyhow::anyhow;
use std::path::PathBuf;

pub fn find_binary(name: &str) -> anyhow::Result<PathBuf> {
    for dir in ["/opt/ooce/bin", "/usr/bin"] {
        let d = PathBuf::from(dir);
        let candidate = d.join(name);

        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(anyhow!("Failed to find {name} binary"))
}
