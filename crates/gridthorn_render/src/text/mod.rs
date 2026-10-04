//! Provisional multilingual layout and DPI-specific presentation snapshots.
mod errors;
mod layout;
mod layout_cache;
mod performance;
mod raster;
mod raster_spans;
mod service;
mod style;

pub use errors::TextError;
pub use layout::{TextGlyph, TextLayout, TextLine, TextMeasurement};
pub use raster::RasterText;
pub use service::TextSystem;
pub use style::{TextAlignment, TextStyle, TextWrap};

#[cfg(test)]
mod test;
