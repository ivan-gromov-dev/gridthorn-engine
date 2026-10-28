//! Provisional aggregate input capabilities and observed raw connections.
mod errors;
mod observation;
mod state;
mod types;

pub use errors::DeviceSelectionError;
pub use observation::RawInput;
pub use state::DeviceState;
pub use types::{DeviceDiscovery, DeviceEvent, InputCapabilities, InputDeviceId, InputSource};

#[cfg(test)]
mod test;
