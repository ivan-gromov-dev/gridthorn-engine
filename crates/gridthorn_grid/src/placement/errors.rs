use thiserror::Error;

use super::GridObjectId;
use crate::GridCell;

/// Invalid footprint or rejected atomic placement operation.
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum PlacementError {
    /// An object must occupy at least one cell.
    #[error("grid footprint must contain at least one offset")]
    EmptyFootprint,
    /// Offsets must be unique.
    #[error("grid footprint repeats offset {0:?}")]
    DuplicateOffset(GridCell),
    /// Translating an offset would exceed signed cell coordinates.
    #[error("placement at {anchor:?} overflows at offset {offset:?}")]
    CellOverflow { anchor: GridCell, offset: GridCell },
    /// The object identity is already placed.
    #[error("grid object {0:?} is already placed")]
    DuplicateObject(GridObjectId),
    /// The requested object has no placement.
    #[error("grid object {0:?} is not placed")]
    MissingObject(GridObjectId),
    /// A candidate cell belongs to another object.
    #[error("placement cell {cell:?} is occupied by {object:?}")]
    Occupied {
        cell: GridCell,
        object: GridObjectId,
    },
}
