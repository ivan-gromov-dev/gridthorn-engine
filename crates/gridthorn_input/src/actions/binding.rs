use crate::controller::{ControllerButton, ControllerId};
use crate::devices::{InputDeviceId, InputSource};
use crate::{InputState, Modifiers, MouseButton, PhysicalKey};

/// Digital input identity. Raw and aggregate streams are deliberately independent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BindingInput {
    /// Layout-independent aggregate key, including native identities.
    Key(PhysicalKey),
    /// Standardized aggregate mouse button.
    MouseButton(MouseButton),
    /// Physical key on one raw connection.
    DeviceKey {
        /// Selected session connection.
        id: InputDeviceId,
        /// Physical identity.
        key: PhysicalKey,
    },
    /// Backend-specific button on one raw connection.
    DeviceButton {
        /// Selected session connection.
        id: InputDeviceId,
        /// Raw button number; not a standardized mouse button.
        button: u32,
    },
    /// Standardized button on one controller connection.
    ControllerButton {
        /// Selected controller connection.
        id: ControllerId,
        /// Physical button position.
        button: ControllerButton,
    },
}

impl BindingInput {
    /// Source selected by this binding; validate it against the current inventory.
    #[must_use]
    pub const fn source(&self) -> InputSource {
        match self {
            Self::Key(_) => InputSource::Keyboard,
            Self::MouseButton(_) => InputSource::Mouse,
            Self::DeviceKey { id, .. } | Self::DeviceButton { id, .. } => InputSource::Device(*id),
            Self::ControllerButton { id, .. } => InputSource::Controller(*id),
        }
    }

    fn down(&self, input: &InputState) -> bool {
        match self {
            Self::Key(key) => input.physical_key_down(key),
            Self::MouseButton(button) => input.mouse_button_down(*button),
            Self::DeviceKey { id, key } => {
                input.device(*id).is_some_and(|device| device.key_down(key))
            }
            Self::DeviceButton { id, button } => input
                .device(*id)
                .is_some_and(|device| device.button_down(*button)),
            Self::ControllerButton { id, button } => input
                .controller(*id)
                .is_some_and(|controller| controller.button_down(*button)),
        }
    }
}

/// One digital binding with optional exact aggregate modifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionBinding {
    /// Selected digital input.
    pub input: BindingInput,
    /// `None` accepts any modifiers; `Some` requires an exact match.
    /// Modifiers always describe aggregate window input, including for raw sources.
    pub modifiers: Option<Modifiers>,
}

impl ActionBinding {
    /// Bind a digital input regardless of modifier state.
    #[must_use]
    pub const fn new(input: BindingInput) -> Self {
        Self {
            input,
            modifiers: None,
        }
    }

    /// Require an exact aggregate modifier combination.
    #[must_use]
    pub const fn with_modifiers(mut self, modifiers: Modifiers) -> Self {
        self.modifiers = Some(modifiers);
        self
    }

    pub(super) fn down(&self, input: &InputState) -> bool {
        self.modifiers
            .is_none_or(|modifiers| modifiers == input.modifiers())
            && self.input.down(input)
    }
}
