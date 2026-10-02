use super::NavigationError;
use crate::GridCell;

/// Inclusive finite search rectangle, independent of presentation projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavigationBounds {
    min: GridCell,
    max: GridCell,
}

impl NavigationBounds {
    /// Creates bounds including both corners.
    ///
    /// # Errors
    /// Rejects reversed bounds on either axis.
    pub fn new(min: GridCell, max: GridCell) -> Result<Self, NavigationError> {
        if min.column > max.column || min.row > max.row {
            return Err(NavigationError::InvalidBounds { min, max });
        }
        Ok(Self { min, max })
    }

    /// Reports whether a cell belongs to the inclusive rectangle.
    #[must_use]
    pub fn contains(self, cell: GridCell) -> bool {
        (self.min.column..=self.max.column).contains(&cell.column)
            && (self.min.row..=self.max.row).contains(&cell.row)
    }
}
