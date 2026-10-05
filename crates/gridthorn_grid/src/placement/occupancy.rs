use std::collections::{BTreeMap, HashMap};

use super::{GridFootprint, PlacementError};
use crate::GridCell;

/// Caller-owned object identity, independent of ECS entity lifetime.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridObjectId(pub u64);

/// Immutable anchor and footprint of one placed object.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GridPlacement {
    anchor: GridCell,
    footprint: GridFootprint,
}

impl GridPlacement {
    /// Authoritative anchor cell.
    #[must_use]
    pub const fn anchor(&self) -> GridCell {
        self.anchor
    }

    /// Occupied offsets relative to the anchor.
    #[must_use]
    pub const fn footprint(&self) -> &GridFootprint {
        &self.footprint
    }
}

/// Sparse exclusive occupancy on an unbounded signed grid.
/// Mutations belong at fixed-tick or explicit load/reset boundaries.
/// Each instance is an independent occupancy space, separate from tile layers.
#[derive(Clone, Debug, Default)]
pub struct PlacementMap {
    objects: BTreeMap<GridObjectId, GridPlacement>,
    cells: HashMap<GridCell, GridObjectId>,
}

impl PlacementMap {
    /// Creates an empty occupancy space.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads the object occupying a cell, including non-anchor footprint cells.
    #[must_use]
    pub fn object_at(&self, cell: GridCell) -> Option<GridObjectId> {
        self.cells.get(&cell).copied()
    }

    /// Reads one object's placement.
    #[must_use]
    pub fn placement(&self, id: GridObjectId) -> Option<&GridPlacement> {
        self.objects.get(&id)
    }

    /// Objects in ascending identity order, independent of insertion order.
    pub fn objects(&self) -> impl ExactSizeIterator<Item = (GridObjectId, &GridPlacement)> {
        self.objects.iter().map(|(id, placement)| (*id, placement))
    }

    /// Checks a new placement or replacement without reserving cells.
    /// An existing object's own cells are ignored, supporting overlapping moves.
    ///
    /// # Errors
    /// Reports overflow before occupancy, then the first conflict in offset order.
    /// Does not check whether the identity exists or validate game terrain rules.
    pub fn validate(
        &self,
        id: GridObjectId,
        anchor: GridCell,
        footprint: &GridFootprint,
    ) -> Result<Vec<GridCell>, PlacementError> {
        let cells = footprint.cells_at(anchor)?;
        for &cell in &cells {
            if let Some(object) = self.object_at(cell)
                && object != id
            {
                return Err(PlacementError::Occupied { cell, object });
            }
        }
        Ok(cells)
    }

    /// Places a new identity after validating its entire footprint.
    ///
    /// # Errors
    /// Rejects duplicate identities, overflow, and occupied cells without mutation.
    pub fn place(
        &mut self,
        id: GridObjectId,
        anchor: GridCell,
        footprint: GridFootprint,
    ) -> Result<(), PlacementError> {
        if self.objects.contains_key(&id) {
            return Err(PlacementError::DuplicateObject(id));
        }
        let cells = self.validate(id, anchor, &footprint)?;
        self.commit(id, anchor, footprint, cells);
        Ok(())
    }

    /// Atomically changes an existing object's anchor and/or footprint.
    ///
    /// # Errors
    /// Rejects missing identities, overflow, and conflicts, preserving old cells.
    pub fn relocate(
        &mut self,
        id: GridObjectId,
        anchor: GridCell,
        footprint: GridFootprint,
    ) -> Result<(), PlacementError> {
        if !self.objects.contains_key(&id) {
            return Err(PlacementError::MissingObject(id));
        }
        let cells = self.validate(id, anchor, &footprint)?;
        self.remove(id);
        self.commit(id, anchor, footprint, cells);
        Ok(())
    }

    /// Removes an object and releases all its cells; missing identities return `None`.
    pub fn remove(&mut self, id: GridObjectId) -> Option<GridPlacement> {
        let placement = self.objects.remove(&id)?;
        for offset in placement.footprint.offsets() {
            self.cells.remove(&GridCell::new(
                placement.anchor.column + offset.column,
                placement.anchor.row + offset.row,
            ));
        }
        Some(placement)
    }

    fn commit(
        &mut self,
        id: GridObjectId,
        anchor: GridCell,
        footprint: GridFootprint,
        cells: Vec<GridCell>,
    ) {
        for cell in cells {
            self.cells.insert(cell, id);
        }
        self.objects.insert(id, GridPlacement { anchor, footprint });
    }
}
