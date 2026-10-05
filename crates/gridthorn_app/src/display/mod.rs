//! Provisional, non-authoritative desktop display observations.
mod catalog;
mod errors;
pub(crate) mod native;
pub(crate) mod selection;
mod snapshot;
mod types;
pub use errors::MonitorSelectionError;

pub use snapshot::Displays;
pub use types::{
    DisplayAvailability, DisplayChange, DisplayMode, DisplayResolution, MonitorId, MonitorInfo,
    MonitorSelection,
};

#[cfg(test)]
mod test;
