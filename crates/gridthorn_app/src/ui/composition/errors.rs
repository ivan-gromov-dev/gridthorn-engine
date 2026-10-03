use super::UiNodeId;

/// Contextual failures leave the tree and its control values unchanged.
#[derive(Debug, thiserror::Error)]
pub enum UiCompositionError {
    /// Invalid dimensions, style metrics or control values.
    #[error("invalid UI metrics: {0}")]
    InvalidMetrics(&'static str),
    /// IDs are unique within one tree.
    #[error("duplicate UI node {0:?}")]
    DuplicateNode(UiNodeId),
    /// Requested node is absent.
    #[error("unknown UI node {0:?}")]
    UnknownNode(UiNodeId),
    /// A command does not apply to this control.
    #[error("command does not apply to UI node {0:?}")]
    WrongControl(UiNodeId),
    /// Tree limits protect recursive layout and painting.
    #[error("UI exceeds 4096 nodes or 64 levels")]
    TooLarge,
    /// Font service diagnostics retain their cause.
    #[error("UI text: {0}")]
    Text(#[from] gridthorn_render::TextError),
    /// Renderer primitive diagnostics retain their cause.
    #[error("UI primitive: {0}")]
    Primitive(#[from] gridthorn_render::UiError),
}
