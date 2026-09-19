mod fixture;
mod lifecycle;
mod publication;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use super::{AssetReloadError, AssetReloader};
use crate::{AssetId, AssetStore};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self::at_timestamp(unique)
    }

    fn at_timestamp(timestamp: u128) -> Self {
        let fixture = loop {
            let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "gridthorn-background-{}-{timestamp}-{sequence}",
                std::process::id()
            ));
            match std::fs::create_dir(&root) {
                Ok(()) => break Self(root),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("could not create fixture '{}': {error}", root.display()),
            }
        };
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
