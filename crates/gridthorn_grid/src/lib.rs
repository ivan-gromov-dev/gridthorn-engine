//! Provisional grid coordinates, sparse tilemaps, object placement, and picking.
//! Integer cells are authoritative; floating-point projections are presentation only.

mod coordinates;
mod errors;
mod placement;
mod projection;
mod tilemap;

pub use coordinates::{GridCell, GridPoint};
pub use errors::GridError;
pub use placement::{GridFootprint, GridObjectId, GridPlacement, PlacementError, PlacementMap};
pub use projection::GridProjection;
pub use tilemap::{ChunkSize, GridView, TileLayer, TileLayerId, TileMap, TileMapError, TilePick};
