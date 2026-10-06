//! Provisional explicit window operations and backend-reported applied state.
mod errors;
pub(crate) mod native;
mod native_result;
mod pending;
mod resource;
mod types;
mod validation;

pub use errors::WindowOperationError;
pub use resource::WindowSettings;
pub use types::{
    WindowCapabilities, WindowMode, WindowModeKind, WindowOperation, WindowPlacement,
    WindowRequest, WindowResizePolicy, WindowState,
};
#[cfg(test)]
mod test;
