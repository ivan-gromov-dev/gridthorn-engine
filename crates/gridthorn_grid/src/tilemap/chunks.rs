use super::TileMapError;
use crate::GridCell;

/// Validated chunk dimensions, independent of tile projection dimensions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChunkSize {
    width: i32,
    height: i32,
}

impl ChunkSize {
    /// Creates a chunk layout without allocating dense cell arrays.
    ///
    /// # Errors
    /// Rejects zero or negative dimensions.
    pub fn new(width: i32, height: i32) -> Result<Self, TileMapError> {
        if width <= 0 || height <= 0 {
            return Err(TileMapError::InvalidChunkSize);
        }
        Ok(Self { width, height })
    }

    /// Width in cells.
    #[must_use]
    pub const fn width(self) -> i32 {
        self.width
    }

    /// Height in cells.
    #[must_use]
    pub const fn height(self) -> i32 {
        self.height
    }

    /// Splits a signed cell into chunk identity and nonnegative local cell.
    /// Negative cells use Euclidean division, including at `i32::MIN`.
    #[must_use]
    pub fn locate(self, cell: GridCell) -> (GridCell, GridCell) {
        (
            GridCell::new(
                cell.column.div_euclid(self.width),
                cell.row.div_euclid(self.height),
            ),
            GridCell::new(
                cell.column.rem_euclid(self.width),
                cell.row.rem_euclid(self.height),
            ),
        )
    }
}
