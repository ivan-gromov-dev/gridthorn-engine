//! Sparse integer tile storage, ordered layers, and projection-based picking.

mod chunks;
mod errors;
mod layers;
mod map;
mod picking;

pub use chunks::ChunkSize;
pub use errors::TileMapError;
pub use layers::{TileLayer, TileLayerId};
pub use map::TileMap;
pub use picking::{GridView, TilePick};

#[cfg(test)]
mod test;
