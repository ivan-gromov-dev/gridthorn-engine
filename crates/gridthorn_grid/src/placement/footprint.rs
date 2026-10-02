use std::collections::BTreeSet;

use super::PlacementError;
use crate::GridCell;

/// Nonempty unique signed offsets relative to an object's anchor.
/// Arbitrary shapes and holes are supported; the anchor need not be occupied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridFootprint {
    offsets: BTreeSet<GridCell>,
}

impl GridFootprint {
    /// Validates offsets and normalizes them into `(column, row)` order.
    ///
    /// # Errors
    /// Rejects empty input and duplicate offsets.
    pub fn new(offsets: impl IntoIterator<Item = GridCell>) -> Result<Self, PlacementError> {
        let mut unique = BTreeSet::new();
        for offset in offsets {
            if !unique.insert(offset) {
                return Err(PlacementError::DuplicateOffset(offset));
            }
        }
        if unique.is_empty() {
            return Err(PlacementError::EmptyFootprint);
        }
        Ok(Self { offsets: unique })
    }

    /// A footprint containing only its anchor cell.
    #[must_use]
    pub fn single_cell() -> Self {
        Self {
            offsets: BTreeSet::from([GridCell::new(0, 0)]),
        }
    }

    /// Offsets in deterministic `(column, row)` order.
    pub fn offsets(&self) -> impl ExactSizeIterator<Item = GridCell> + '_ {
        self.offsets.iter().copied()
    }

    /// Resolves the whole footprint without changing any placement state.
    /// Games can use these cells for terrain, bounds, and preview validation.
    ///
    /// # Errors
    /// Rejects any translated cell outside the signed `i32` range.
    pub fn cells_at(&self, anchor: GridCell) -> Result<Vec<GridCell>, PlacementError> {
        self.offsets()
            .map(|offset| {
                let column = anchor.column.checked_add(offset.column);
                let row = anchor.row.checked_add(offset.row);
                match (column, row) {
                    (Some(column), Some(row)) => Ok(GridCell::new(column, row)),
                    _ => Err(PlacementError::CellOverflow { anchor, offset }),
                }
            })
            .collect()
    }
}
