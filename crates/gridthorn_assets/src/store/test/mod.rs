mod dependencies;
mod reload;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{AssetId, AssetStore, AssetStoreError};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let directory = std::env::temp_dir().join(format!(
            "gridthorn-assets-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&directory).unwrap();
        Self(directory)
    }

    fn write(&self, name: &str, bytes: &[u8]) {
        std::fs::write(self.0.join(name), bytes).unwrap();
    }

    fn texture(&self, name: &str, rgb: [u8; 3]) {
        let mut bytes = b"P6\n1 1\n255\n".to_vec();
        bytes.extend(rgb);
        self.write(name, &bytes);
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn id(path: &str) -> AssetId {
    AssetId::new(path).unwrap()
}
