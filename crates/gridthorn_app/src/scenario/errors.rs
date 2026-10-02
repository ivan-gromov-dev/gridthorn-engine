use thiserror::Error;

use crate::LifecycleError;

/// Recoverable scenario construction, capture, or restoration failure.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ScenarioError {
    /// Scenario names are explicit compatibility identities.
    #[error("scenario identity must be nonempty, unpadded, and have a positive revision")]
    InvalidIdentity,
    /// The game removed the required authoritative root resource.
    #[error("scenario authoritative state resource is missing")]
    MissingState,
    /// Restoration requires the same scenario, revision, engine, and configuration.
    #[error("snapshot is incompatible: {0}")]
    Incompatible(&'static str),
    /// The runtime cannot accept more work.
    #[error(transparent)]
    Lifecycle(#[from] LifecycleError),
}
