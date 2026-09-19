use std::path::PathBuf;

use thiserror::Error;

use super::AssetId;
use crate::TextureAssetError;

/// Contextual failure that leaves the store's last committed state intact.
#[derive(Debug, Error)]
pub enum AssetStoreError {
    /// Asset identities must be portable relative paths.
    #[error(
        "invalid asset ID '{path}': expected a relative forward-slash path without dot or empty segments"
    )]
    InvalidId {
        /// Rejected path.
        path: String,
    },
    /// An ID has not been registered in this store.
    #[error("asset '{id}' is not registered")]
    Unknown {
        /// Missing identity.
        id: AssetId,
    },
    /// Registration cannot silently change an asset's kind or data.
    #[error("asset '{id}' is already registered")]
    Duplicate {
        /// Existing identity.
        id: AssetId,
    },
    /// The requested dependency list introduces a cycle.
    #[error("dependencies of asset '{id}' would introduce a cycle")]
    DependencyCycle {
        /// Asset whose dependency change was rejected.
        id: AssetId,
    },
    /// A source could not be read; retry after repairing the file.
    #[error("could not read asset '{id}' at '{}': {source}", path.display())]
    Read {
        /// Asset identity.
        id: AssetId,
        /// Resolved source path.
        path: PathBuf,
        /// Filesystem failure.
        #[source]
        source: std::io::Error,
    },
    /// A texture source could not be decoded.
    #[error("could not prepare asset '{id}': {source}")]
    Texture {
        /// Asset identity.
        id: AssetId,
        /// Contextual decoder failure.
        #[source]
        source: TextureAssetError,
    },
}
