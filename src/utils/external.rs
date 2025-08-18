use anyhow::anyhow;
use camino::Utf8PathBuf;

pub fn find_binary(name: &str) -> anyhow::Result<Utf8PathBuf> {
    for dir in [
        "/opt/ooce/bin",
        "/usr/bin",
        "/home/rob/bin/SunOS",
        "/opt/homebrew/bin",
    ] {
        let d = Utf8PathBuf::from(dir);
        let candidate = d.join(name);

        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(anyhow!("Failed to find {name} binary"))
}
