use std::path::PathBuf;
use thiserror::Error;

use crate::ScenarioError;

/// Recoverable document, compatibility, game-codec, or filesystem failure.
#[derive(Debug, Error)]
pub enum WorldSaveError {
    /// The envelope is malformed or unsupported.
    #[error("invalid world save: {0}")]
    Document(String),
    /// Game data failed encoding or validation.
    #[error("world save game payload: {0}")]
    Codec(String),
    /// A filesystem operation failed before replacement or load application.
    #[error("world save {operation} at {path}: {source}")]
    Io {
        /// Operation that failed.
        operation: &'static str,
        /// File involved in the failure.
        path: PathBuf,
        /// Underlying filesystem diagnostic.
        #[source]
        source: std::io::Error,
    },
    /// Runtime compatibility or lifecycle rejection.
    #[error(transparent)]
    Scenario(#[from] ScenarioError),
}
