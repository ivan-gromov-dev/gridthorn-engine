use std::path::{Path, PathBuf};

use super::{WatchError, snapshot::SourceSnapshot};
use crate::commands::check;

/// The most recent validation/compiler outcome; failed checks do not end a session.
pub(crate) enum CheckOutcome {
    Passed,
    Failed(String),
}

/// One synchronous check at a time, with complete snapshots committed before checking.
pub(crate) struct WatchSession {
    root: PathBuf,
    snapshot: SourceSnapshot,
}

impl WatchSession {
    pub(crate) fn start(root: &Path) -> Result<(Self, CheckOutcome), WatchError> {
        let target_directory = target_directory(root);
        let snapshot = SourceSnapshot::scan(root, target_directory.as_deref())?;
        let outcome = run_check(root);
        Ok((
            Self {
                root: root.into(),
                snapshot,
            },
            outcome,
        ))
    }

    pub(crate) fn poll(&mut self) -> Result<Option<CheckOutcome>, WatchError> {
        let target_directory = target_directory(&self.root);
        let next = SourceSnapshot::scan(&self.root, target_directory.as_deref())?;
        if next == self.snapshot {
            return Ok(None);
        }
        self.snapshot = next;
        Ok(Some(run_check(&self.root)))
    }
}

fn target_directory(root: &Path) -> Option<PathBuf> {
    let path = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from)?;
    let path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir().ok()?.join(path)
    };
    path.canonicalize().ok().filter(|path| path != root)
}

fn run_check(root: &Path) -> CheckOutcome {
    match check::execute(root) {
        Ok(()) => CheckOutcome::Passed,
        Err(error) => CheckOutcome::Failed(format!("{error:#}")),
    }
}
