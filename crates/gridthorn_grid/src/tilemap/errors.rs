use thiserror::Error;

use super::TileLayerId;
use crate::GridError;

/// Invalid tilemap configuration, layer access, or presentation query.
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum TileMapError {
    /// Chunk dimensions must be strictly positive signed integers.
    #[error("tilemap chunk width and height must be positive")]
    InvalidChunkSize,
    /// A layer identifier is already registered.
    #[error("tilemap layer {0:?} already exists")]
    DuplicateLayer(TileLayerId),
    /// An operation references a layer that has not been registered.
    #[error("tilemap layer {0:?} does not exist")]
    MissingLayer(TileLayerId),
    /// Camera extent and viewport pixel dimensions must be finite and positive.
    #[error("grid view extent and viewport dimensions must be finite and positive")]
    InvalidView,
    /// A coordinate conversion failed.
    #[error("tilemap picking: {0}")]
    Projection(#[from] GridError),
}
