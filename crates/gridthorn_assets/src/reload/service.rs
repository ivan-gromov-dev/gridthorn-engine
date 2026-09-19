use std::sync::{Mutex, mpsc};
use std::thread::{self, JoinHandle};

use super::{AssetReloadError, worker};
use crate::{AssetId, AssetStore, AssetStoreError};

/// Provisional background content polling with explicit frame-boundary commits.
///
/// Construct a store and its dependency graph before transferring it here.
/// Registration and dependencies are fixed for this worker's lifetime. File
/// reads and decoding run on one owned thread; `poll` applies a completed batch
/// without waiting for I/O. The visible store changes only when `poll` succeeds.
/// Existing snapshots remain immutable. This service is `Send + Sync` and its
/// driving methods require exclusive mutable access.
///
/// There is at most one outstanding request/result. Repeated requests while
/// busy return `false` and are not queued. Callers choose their polling cadence
/// and must request again after consuming a result to detect subsequent edits.
/// This is a background content poller, not an operating-system file watcher.
/// `shutdown` and dropping this service join the worker and may wait for active
/// file I/O. Use shutdown outside latency-sensitive frame processing.
pub struct AssetReloader {
    assets: AssetStore,
    requests: Option<mpsc::Sender<()>>,
    replies: Mutex<mpsc::Receiver<worker::Reply>>,
    worker: Option<JoinHandle<()>>,
    pending: bool,
}

impl AssetReloader {
    /// Start one worker with the supplied store's registered assets and graph.
    ///
    /// Initial loading remains synchronous. No poll is started automatically.
    /// Prefer an absolute store root; changing the process working directory
    /// also affects relative paths used by the worker.
    ///
    /// # Errors
    /// Returns a contextual error if thread creation fails.
    pub fn new(assets: AssetStore) -> Result<Self, AssetReloadError> {
        Self::start(assets, AssetStore::reload_changed)
    }

    /// The last frame-boundary committed store, available while work is pending.
    #[must_use]
    pub fn assets(&self) -> &AssetStore {
        &self.assets
    }

    /// Whether a request is running or its result awaits consumption by `poll`.
    #[must_use]
    pub fn is_pending(&self) -> bool {
        self.pending
    }

    /// Request one content scan without waiting for file reads or decoding.
    ///
    /// Returns `false` while an earlier request/result is outstanding. Such a
    /// request is not remembered; request again after `poll` consumes the result.
    ///
    /// # Errors
    /// Returns `Stopped` after shutdown or unexpected worker termination.
    pub fn request_reload(&mut self) -> Result<bool, AssetReloadError> {
        let sender = self.requests.as_ref().ok_or(AssetReloadError::Stopped)?;
        if self.pending {
            return Ok(false);
        }
        sender.send(()).map_err(|_| AssetReloadError::Stopped)?;
        self.pending = true;
        Ok(true)
    }

    /// Apply a ready batch at the caller's chosen presentation frame boundary.
    ///
    /// `None` means no result is ready; `Some` contains dependency-ordered IDs
    /// and may be empty if contents were unchanged. No file I/O or decoding runs
    /// here. Swapping snapshots may release old data and is not a hard real-time
    /// guarantee. Never adopt authoritative game data through this service.
    ///
    /// # Errors
    /// Preparation errors preserve the visible store and allow another request.
    /// Worker termination is reported as `Stopped`; recreate the service to retry.
    pub fn poll(&mut self) -> Result<Option<Vec<AssetId>>, AssetReloadError> {
        if self.requests.is_none() {
            return Err(AssetReloadError::Stopped);
        }
        let result = self
            .replies
            .get_mut()
            .map_err(|_| AssetReloadError::Stopped)?
            .try_recv();
        match result {
            Ok(reply) => {
                self.pending = false;
                let prepared = reply.map_err(|source| AssetReloadError::Prepare { source })?;
                if let Some(snapshot) = prepared.snapshot {
                    self.assets = snapshot;
                }
                Ok(Some(prepared.changed))
            }
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => {
                self.pending = false;
                Err(AssetReloadError::Stopped)
            }
        }
    }

    /// Stop accepting requests, join the worker, and discard any unapplied batch.
    ///
    /// The committed store remains readable. This is idempotent and may block
    /// until an active read/decode finishes; disk I/O is not cancellable.
    ///
    /// # Errors
    /// Returns `Stopped` if the worker panicked while running.
    pub fn shutdown(&mut self) -> Result<(), AssetReloadError> {
        self.requests.take();
        self.pending = false;
        if let Some(worker) = self.worker.take() {
            worker.join().map_err(|_| AssetReloadError::Stopped)?;
        }
        if let Ok(replies) = self.replies.get_mut() {
            while replies.try_recv().is_ok() {}
        }
        Ok(())
    }

    pub(super) fn start(
        assets: AssetStore,
        reload: impl FnMut(&mut AssetStore) -> Result<Vec<AssetId>, AssetStoreError> + Send + 'static,
    ) -> Result<Self, AssetReloadError> {
        let snapshot = assets.snapshot();
        let (requests, incoming) = mpsc::channel();
        let (outgoing, replies) = mpsc::sync_channel(1);
        let worker = thread::Builder::new()
            .name("gridthorn-asset-reload".to_owned())
            .spawn(move || worker::run(assets, &incoming, &outgoing, reload))
            .map_err(|source| AssetReloadError::Start { source })?;
        Ok(Self {
            assets: snapshot,
            requests: Some(requests),
            replies: Mutex::new(replies),
            worker: Some(worker),
            pending: false,
        })
    }
}

impl Drop for AssetReloader {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
