/// Recoverable source selection failure.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DeviceSelectionError {
    /// Observation must be explicitly enabled before selecting a raw source.
    #[error("raw input observation is disabled")]
    Disabled,
    /// Adapter cannot select individual raw devices.
    #[error("individual raw input devices are unsupported by this adapter")]
    Unsupported,
    /// Source has not been reported or its connection ended.
    #[error("input source {0:?} is unavailable in the current inventory")]
    Unavailable(super::InputSource),
}
