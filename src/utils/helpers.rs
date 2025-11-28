use anyhow::ensure;
use camino::Utf8Path;
use indicatif::ProgressBar;

pub fn check_hierarchy(root: &Utf8Path) -> anyhow::Result<()> {
    let mp3_root = root.join("mp3");
    let flac_root = root.join("flac");

    ensure!(mp3_root.exists(), format!("did not find {}", mp3_root));
    ensure!(flac_root.exists(), format!("did not find {}", flac_root));

    Ok(())
}

// Extend Indicatif's progress bars so we can opt in and out of them. e.g. we want a bar only when
// there's --recurse
//
pub enum MaybeProgress {
    Bar(ProgressBar),
    Direct,
}

impl MaybeProgress {
    pub fn inc(&self, delta: u64) {
        if let MaybeProgress::Bar(pb) = self {
            pb.inc(delta);
        }
    }

    pub fn println(&self, msg: &str) {
        match self {
            MaybeProgress::Bar(pb) => pb.println(msg),
            MaybeProgress::Direct => println!("{}", msg),
        }
    }

    pub fn finish(&self) {
        if let MaybeProgress::Bar(pb) = self {
            pb.finish();
        }
    }
}
