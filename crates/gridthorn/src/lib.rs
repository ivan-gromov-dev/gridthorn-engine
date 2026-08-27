//! Public SDK facade for Gridthorn.
//!
//! The runtime is still under construction. This crate currently exposes only
//! compatibility information used by generated projects and the CLI.

mod version;

pub use version::version;

/// Commonly used Gridthorn APIs.
///
/// This prelude is intentionally empty until the first runtime APIs are
/// implemented. Keeping it present lets generated projects adopt APIs without
/// changing their import convention later.
pub mod prelude {}
