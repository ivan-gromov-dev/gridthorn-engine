use crate::composition::{UiCompositionError, UiNodeId};

/// Animation failures preserve both the animation and its destination tree.
#[derive(Debug, thiserror::Error)]
pub enum UiAnimationError {
    /// Interpolation endpoints must be finite and property bounds must hold.
    #[error("invalid UI animation values: {0}")]
    InvalidValue(&'static str),
    /// A transition cannot change its property kind.
    #[error("UI transition endpoints have different property kinds")]
    PropertyMismatch,
    /// Applying a sample failed; the destination node identifies the operation.
    #[error("UI animation for node {node:?}: {source}")]
    Destination {
        /// Destination node.
        node: UiNodeId,
        /// Composition failure.
        #[source]
        source: UiCompositionError,
    },
}
