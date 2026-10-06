//! Provisional surface policy, request feedback and monotonic frame pacing.
mod errors;
pub(in crate::window) mod native;
mod pacing;
mod resource;
mod types;
pub use errors::PresentationError;
pub use gridthorn_render::PresentMode;
pub(super) use pacing::FramePacer;
pub use resource::PresentationSettings;
pub use types::{FrameRateLimit, PresentationConfig, PresentationOperation, PresentationState};

#[cfg(test)]
mod test;
