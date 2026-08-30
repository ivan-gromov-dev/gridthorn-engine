use thiserror::Error;

use gridthorn_simulation::TimeError;

/// Failure while advancing the application lifecycle.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum LifecycleError {
    /// A frame was requested after the runtime completed shutdown.
    #[error("cannot run an application frame after shutdown")]
    AlreadyShutdown,
    /// The fixed-step clock could not represent the requested advancement.
    #[error(transparent)]
    Time(#[from] TimeError),
}
