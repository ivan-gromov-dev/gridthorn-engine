use super::*;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Rejected;

impl WorldSaveCodec<Vec<u64>, u64> for Rejected {
    fn encode(&self, _: &Vec<u64>, _: &GameCommandQueue<u64>) -> Result<String, String> {
        Err("invalid game state".to_owned())
    }
    fn decode(&self, _: &str) -> Result<(Vec<u64>, GameCommandQueue<u64>), String> {
        unreachable!()
    }
}

struct Directory(std::path::PathBuf);

impl Directory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "gridthorn-save-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for Directory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn files_replace_existing_save_and_restore_in_fresh_runtime() {
    let directory = Directory::new();
    let path = directory.0.join("world.toml");
    let mut source = runtime();
    source.save_file(&path, &Codec).unwrap();
    source.run_ticks(5).unwrap();
    source.save_file(&path, &Codec).unwrap();
    let mut target = runtime();
    target.load_file(&path, &Codec).unwrap();
    assert_eq!(
        source.save_document(&Codec).unwrap(),
        target.save_document(&Codec).unwrap()
    );
    assert_eq!(fs::read_dir(&directory.0).unwrap().count(), 1);
    let previous = fs::read(&path).unwrap();
    assert!(source.save_file(&path, &Rejected).is_err());
    assert_eq!(previous, fs::read(&path).unwrap());
}

#[test]
fn failed_replacement_cleans_temporary_and_failed_load_preserves_live_state() {
    let directory = Directory::new();
    let destination = directory.0.join("directory");
    fs::create_dir(&destination).unwrap();
    let mut target = runtime();
    assert!(matches!(
        target.save_file(&destination, &Codec),
        Err(WorldSaveError::Io { .. })
    ));
    assert_eq!(fs::read_dir(&directory.0).unwrap().count(), 1);
    let valid = target.save_document(&Codec).unwrap();
    assert!(
        target
            .load_file(directory.0.join("missing"), &Codec)
            .is_err()
    );
    let path = directory.0.join("invalid");
    fs::write(&path, [0xff]).unwrap();
    assert!(target.load_file(&path, &Codec).is_err());
    let file = fs::File::create(&path).unwrap();
    file.set_len(16 * 1024 * 1024 + 1).unwrap();
    assert!(target.load_file(&path, &Codec).is_err());
    assert_eq!(target.save_document(&Codec).unwrap(), valid);
}
