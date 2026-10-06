//! Provisional controller identities, frame state, dead zones and feedback requests.
mod dead_zone;
mod errors;
mod feedback;
mod polling;
pub use polling::ControllerPolling;
mod state;
mod types;
pub use dead_zone::DeadZone;
pub use errors::ControllerError;
pub use feedback::{ControllerFeedback, RumbleRequest};
pub use state::ControllerState;
pub use types::*;
#[cfg(test)]
mod test;
