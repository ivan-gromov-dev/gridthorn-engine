use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

static WORKSPACE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(crate) struct ProjectWorkspace {
    pub(crate) root: PathBuf,
}

impl ProjectWorkspace {
    pub(crate) fn create() -> Result<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self::at_timestamp(timestamp, &WORKSPACE_SEQUENCE)
    }

    pub(super) fn at_timestamp(timestamp: u128, sequence: &AtomicU64) -> Result<Self> {
        loop {
            let index = sequence.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "gridthorn-cli-command-test-{}-{timestamp}-{index}",
                std::process::id()
            ));
            match fs::create_dir(&root) {
                Ok(()) => return Ok(Self { root }),
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!("failed to create test workspace {}", root.display())
                    });
                }
            }
        }
    }

    pub(super) fn project(&self) -> PathBuf {
        self.root.join("minimal-game")
    }
}

impl Drop for ProjectWorkspace {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.root)
            && self.root.exists()
        {
            eprintln!(
                "failed to remove command test workspace {}: {error}",
                self.root.display()
            );
        }
    }
}
