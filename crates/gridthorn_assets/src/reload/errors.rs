use thiserror::Error;

use crate::AssetStoreError;

/// Failure to start, prepare, or receive a background asset reload.
#[derive(Debug, Error)]
pub enum AssetReloadError {
    /// The operating system could not start the worker thread.
    #[error("could not start asset reload worker: {source}")]
    Start {
        /// Underlying thread creation failure.
        #[source]
        source: std::io::Error,
    },
    /// The worker has shut down or terminated unexpectedly.
    #[error("asset reload worker stopped; create a new reloader to resume loading")]
    Stopped,
    /// Preparation failed; committed data is intact and another request may retry.
    #[error("background asset reload failed: {source}")]
    Prepare {
        /// Contextual file or decoding failure.
        #[source]
        source: AssetStoreError,
    },
}
