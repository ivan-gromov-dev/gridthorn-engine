use std::collections::{BTreeMap, btree_map::Entry};

use super::{ChunkSize, TileLayer, TileLayerId, TileMapError};
use crate::GridCell;

/// Unbounded sparse tilemap with deterministic layer and chunk iteration.
/// Integer storage can be authoritative; callers mutate it at fixed-tick or
/// explicit load/reset boundaries. No lifecycle systems are registered implicitly.
#[derive(Clone, Debug)]
pub struct TileMap<T> {
    size: ChunkSize,
    layers: BTreeMap<TileLayerId, TileLayer<T>>,
}

impl<T> TileMap<T> {
    /// Creates an empty map with one shared chunk layout for all layers.
    #[must_use]
    pub fn new(size: ChunkSize) -> Self {
        Self {
            size,
            layers: BTreeMap::new(),
        }
    }

    /// Chunk layout used by this map.
    #[must_use]
    pub const fn chunk_size(&self) -> ChunkSize {
        self.size
    }

    /// Adds an empty visible, pickable layer.
    ///
    /// # Errors
    /// Rejects duplicate identifiers without changing the existing layer.
    pub fn add_layer(&mut self, id: TileLayerId) -> Result<(), TileMapError> {
        match self.layers.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert(TileLayer::new());
                Ok(())
            }
            Entry::Occupied(_) => Err(TileMapError::DuplicateLayer(id)),
        }
    }

    /// Removes a layer and returns all its stored data, if it existed.
    pub fn remove_layer(&mut self, id: TileLayerId) -> Option<TileLayer<T>> {
        self.layers.remove(&id)
    }

    /// Layers in ascending identity order, from bottom to top.
    pub fn layers(&self) -> impl DoubleEndedIterator<Item = (TileLayerId, &TileLayer<T>)> {
        self.layers.iter().map(|(id, layer)| (*id, layer))
    }

    /// Reads a registered layer.
    #[must_use]
    pub fn layer(&self, id: TileLayerId) -> Option<&TileLayer<T>> {
        self.layers.get(&id)
    }

    /// Changes flags on a registered layer.
    pub fn layer_mut(&mut self, id: TileLayerId) -> Option<&mut TileLayer<T>> {
        self.layers.get_mut(&id)
    }

    /// Reads a tile; missing layers and empty cells return `None`.
    #[must_use]
    pub fn tile(&self, id: TileLayerId, cell: GridCell) -> Option<&T> {
        self.layer(id)?.tile(self.size, cell)
    }

    /// Inserts/replaces a tile and returns the previous value. Passing `None`
    /// clears the cell and releases its chunk when the last tile is removed.
    ///
    /// # Errors
    /// Rejects an unregistered layer before changing storage.
    pub fn set_tile(
        &mut self,
        id: TileLayerId,
        cell: GridCell,
        tile: Option<T>,
    ) -> Result<Option<T>, TileMapError> {
        let layer = self
            .layers
            .get_mut(&id)
            .ok_or(TileMapError::MissingLayer(id))?;
        let (chunk, local) = self.size.locate(cell);
        if let Some(tile) = tile {
            return Ok(layer
                .chunks
                .entry(chunk)
                .or_default()
                .insert(local, (cell, tile))
                .map(|(_, tile)| tile));
        }
        let Some(tiles) = layer.chunks.get_mut(&chunk) else {
            return Ok(None);
        };
        let removed = tiles.remove(&local).map(|(_, tile)| tile);
        if tiles.is_empty() {
            layer.chunks.remove(&chunk);
        }
        Ok(removed)
    }
}
