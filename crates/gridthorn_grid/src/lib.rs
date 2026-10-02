//! Provisional grid coordinates and presentation projections.
//! Integer cells are authoritative; floating-point projections are presentation only.

mod coordinates;
mod errors;
mod projection;

pub use coordinates::{GridCell, GridPoint};
pub use errors::GridError;
pub use projection::GridProjection;
