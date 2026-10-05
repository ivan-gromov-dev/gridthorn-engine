use super::*;
use std::cell::Cell;
use std::rc::Rc;

struct Resource(Rc<Cell<usize>>);

impl Drop for Resource {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

fn texture() -> TextureAsset {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("gridthorn-cache-{unique}.ppm"));
    std::fs::write(&path, b"P6\n1 1\n255\n\xff\x00\x00").unwrap();
    let asset = TextureAsset::load(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    asset
}

#[test]
fn cloned_handles_reuse_resources_and_absent_or_reloaded_identities_release_them() {
    let first = texture();
    let replacement = texture();
    let dropped = Rc::new(Cell::new(0));
    let mut cache = TextureCache::new();
    assert!(cache.get_or_insert(&first, || Resource(dropped.clone())).1);
    assert!(
        !cache
            .get_or_insert(&first.clone(), || panic!("clone must reuse"))
            .1
    );
    assert!(
        cache
            .get_or_insert(&replacement, || Resource(dropped.clone()))
            .1
    );
    assert_eq!(cache.len(), 2);
    assert_eq!(cache.bytes(), first.rgba8().len() * 2);
    cache.retain(&[&replacement]);
    assert_eq!(dropped.get(), 1);
    cache.retain(&[]);
    assert_eq!(dropped.get(), 2);
    assert_eq!(cache.bytes(), 0);
    assert!(cache.get_or_insert(&first, || Resource(dropped.clone())).1);
    drop(cache);
    assert_eq!(dropped.get(), 3);
}
