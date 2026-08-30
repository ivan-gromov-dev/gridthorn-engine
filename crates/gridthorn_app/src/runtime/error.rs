use thiserror::Error;

/// Failure while advancing the application lifecycle.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum LifecycleError {
    /// A frame was requested after the runtime completed shutdown.
    #[error("cannot run an application frame after shutdown")]
    AlreadyShutdown,
}
