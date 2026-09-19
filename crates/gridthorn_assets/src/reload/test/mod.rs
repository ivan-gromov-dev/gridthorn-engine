mod lifecycle;
mod publication;

use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::{AssetReloadError, AssetReloader};
use crate::{AssetId, AssetStore};

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "gridthorn-background-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir(&root).unwrap();
        let fixture = Self(root);
        fixture.write("sprite.ppm", b"P3\n1 1\n255\n255 0 0\n");
        fixture.write("scene.txt", b"sprite.ppm");
        fixture
    }

    fn write(&self, name: &str, bytes: &[u8]) {
        std::fs::write(self.0.join(name), bytes).unwrap();
    }

    fn store(&self) -> AssetStore {
        let mut store = AssetStore::new(&self.0);
        store.load_texture(id("sprite.ppm")).unwrap();
        store.load_source(id("scene.txt")).unwrap();
        store
            .set_dependencies(&id("scene.txt"), &[id("sprite.ppm")])
            .unwrap();
        store
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

fn finish(reloader: &mut AssetReloader) -> Result<Vec<AssetId>, AssetReloadError> {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(changed) = reloader.poll()? {
            return Ok(changed);
        }
        assert!(Instant::now() < deadline, "worker did not finish");
        std::thread::sleep(Duration::from_millis(1));
    }
}
