mod errors;
mod footprint;
mod occupancy;

pub use errors::PlacementError;
pub use footprint::GridFootprint;
pub use occupancy::{GridObjectId, GridPlacement, PlacementMap};

#[cfg(test)]
mod test;
