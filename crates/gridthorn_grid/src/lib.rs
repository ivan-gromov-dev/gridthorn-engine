//! Provisional grid coordinates, sparse tilemaps, and presentation picking.
//! Integer cells are authoritative; floating-point projections are presentation only.

mod coordinates;
mod errors;
mod projection;
mod tilemap;

pub use coordinates::{GridCell, GridPoint};
pub use errors::GridError;
pub use projection::GridProjection;
pub use tilemap::{ChunkSize, GridView, TileLayer, TileLayerId, TileMap, TileMapError, TilePick};
