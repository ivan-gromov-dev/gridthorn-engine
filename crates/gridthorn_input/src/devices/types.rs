use crate::{ButtonState, PhysicalKey, controller::ControllerId};

/// Session connection identity; never persist across application runs or reconnects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputDeviceId(pub u64);

/// Adapter discovery contract, distinct from OS hardware availability.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DeviceDiscovery {
    /// No adapter has reported capabilities.
    #[default]
    Unknown,
    /// Individual raw devices are unavailable through this adapter.
    Unsupported,
    /// Connections appear on arrival or first input; no complete initial inventory.
    Observed,
}

/// Virtual aggregate sources and explicit raw-device discovery support.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InputCapabilities {
    /// Window keyboard stream is available; not a count of physical keyboards.
    pub aggregate_keyboard: bool,
    /// Window pointer stream is available; not a count of physical mice.
    pub aggregate_mouse: bool,
    /// Raw-device discovery and selection contract.
    pub raw_devices: DeviceDiscovery,
}

/// Selectable binding source; selection never redirects OS focus, cursor or IME.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputSource {
    /// All keyboards contributing to window input.
    Keyboard,
    /// All pointers contributing to window input.
    Mouse,
    /// One observed raw connection; button numbers are backend-specific.
    Device(InputDeviceId),
    /// One discovered controller connection.
    Controller(ControllerId),
}

impl InputSource {
    /// Validate against this frame's inventory. Disconnect invalidates selection.
    ///
    /// # Errors
    /// Returns an unavailable or unsupported result without choosing a fallback.
    pub fn validate(self, input: &crate::InputState) -> Result<(), super::DeviceSelectionError> {
        use super::DeviceSelectionError;
        let available = match self {
            Self::Keyboard => input.input_capabilities().aggregate_keyboard,
            Self::Mouse => input.input_capabilities().aggregate_mouse,
            Self::Device(id) => {
                if input.input_capabilities().raw_devices == DeviceDiscovery::Unsupported {
                    return Err(DeviceSelectionError::Unsupported);
                }
                if !input.raw_input_enabled() {
                    return Err(DeviceSelectionError::Disabled);
                }
                input.device(id).is_some()
            }
            Self::Controller(id) => input.controller(id).is_some(),
        };
        if available {
            Ok(())
        } else {
            Err(DeviceSelectionError::Unavailable(self))
        }
    }
}

/// Raw lifecycle and input, independent of aggregate window events.
#[derive(Clone, Debug, PartialEq)]
pub enum DeviceEvent {
    /// Arrival or first observed input. Input capabilities accumulate from events.
    Connected(InputDeviceId),
    /// Connection removed; all held state is cancelled.
    Disconnected(InputDeviceId),
    /// Physical keyboard transition without layout, text or IME semantics.
    Key {
        /// Connection identity.
        id: InputDeviceId,
        /// Physical identity.
        key: PhysicalKey,
        /// New digital state; duplicate presses are not new edges.
        state: ButtonState,
    },
    /// Backend-specific raw button; not a standardized window mouse button.
    Button {
        /// Connection identity.
        id: InputDeviceId,
        /// Backend button number.
        button: u32,
        /// New digital state.
        state: ButtonState,
    },
    /// Relative pointer displacement while focused, independent of capture.
    Motion {
        /// Connection identity.
        id: InputDeviceId,
        /// Horizontal displacement in backend units.
        x: f64,
        /// Vertical displacement in backend units.
        y: f64,
    },
}
