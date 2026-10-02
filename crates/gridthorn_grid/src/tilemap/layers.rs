use std::collections::BTreeMap;

use super::ChunkSize;
use crate::GridCell;

/// Stable caller-selected layer identity; larger values are above smaller ones.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TileLayerId(pub i32);

/// Sparse layer with independent visibility and picking flags, both initially true.
/// Tiles own caller-defined data, without renderer or entity handles imposed by the SDK.
#[derive(Clone, Debug)]
pub struct TileLayer<T> {
    pub(super) chunks: BTreeMap<GridCell, BTreeMap<GridCell, (GridCell, T)>>,
    visible: bool,
    pickable: bool,
}

impl<T> TileLayer<T> {
    pub(super) fn new() -> Self {
        Self {
            chunks: BTreeMap::new(),
            visible: true,
            pickable: true,
        }
    }

    /// Whether presentation and picking should include this layer.
    #[must_use]
    pub const fn is_visible(&self) -> bool {
        self.visible
    }

    /// Changes presentation visibility; stored tiles are preserved.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Whether tilemap picking may return this layer.
    #[must_use]
    pub const fn is_pickable(&self) -> bool {
        self.pickable
    }

    /// Changes picking eligibility without changing presentation visibility.
    pub fn set_pickable(&mut self, pickable: bool) {
        self.pickable = pickable;
    }

    /// Occupied chunks in ascending `(column, row)` order, with tile counts.
    pub fn chunks(&self) -> impl DoubleEndedIterator<Item = (GridCell, usize)> + '_ {
        self.chunks.iter().map(|(cell, tiles)| (*cell, tiles.len()))
    }

    /// Tiles ordered by chunk, then local `(column, row)`; this is storage order,
    /// not an isometric sprite depth order. Returned cells are global identities.
    pub fn tiles(&self) -> impl Iterator<Item = (GridCell, &T)> {
        self.chunks
            .values()
            .flat_map(|tiles| tiles.values().map(|(cell, tile)| (*cell, tile)))
    }

    /// Reads only one occupied chunk in local `(column, row)` order.
    /// Missing chunks yield an empty iterator; returned cells are global identities.
    pub fn chunk_tiles(&self, chunk: GridCell) -> impl Iterator<Item = (GridCell, &T)> {
        self.chunks
            .get(&chunk)
            .into_iter()
            .flat_map(|tiles| tiles.values().map(|(cell, tile)| (*cell, tile)))
    }

    pub(super) fn tile(&self, size: ChunkSize, cell: GridCell) -> Option<&T> {
        let (chunk, local) = size.locate(cell);
        self.chunks.get(&chunk)?.get(&local).map(|(_, tile)| tile)
    }
}
