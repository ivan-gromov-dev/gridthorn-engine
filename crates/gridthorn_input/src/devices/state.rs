use std::collections::BTreeSet;

use super::{DeviceEvent, InputDeviceId};
use crate::{ButtonState, PhysicalKey};

/// Observed connection; flags describe observed input, not exhaustive hardware support.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceState {
    /// Session connection identity.
    pub id: InputDeviceId,
    /// At least one raw keyboard event has arrived.
    pub keyboard_observed: bool,
    /// At least one raw button event has arrived.
    pub buttons_observed: bool,
    /// At least one raw relative motion event has arrived.
    pub motion_observed: bool,
    keys: BTreeSet<PhysicalKey>,
    buttons: BTreeSet<u32>,
}

impl DeviceState {
    pub(crate) fn new(id: InputDeviceId) -> Self {
        Self {
            id,
            keyboard_observed: false,
            buttons_observed: false,
            motion_observed: false,
            keys: BTreeSet::new(),
            buttons: BTreeSet::new(),
        }
    }

    /// Whether a raw physical key is held.
    #[must_use]
    pub fn key_down(&self, key: &PhysicalKey) -> bool {
        self.keys.contains(key)
    }

    /// Whether a backend-specific button is held.
    #[must_use]
    pub fn button_down(&self, button: u32) -> bool {
        self.buttons.contains(&button)
    }

    pub(crate) fn cancel(&mut self) {
        self.keys.clear();
        self.buttons.clear();
    }

    pub(crate) fn apply(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::Key { key, state, .. } => {
                self.keyboard_observed = true;
                if *state == ButtonState::Pressed {
                    self.keys.insert(key.clone());
                } else {
                    self.keys.remove(key);
                }
            }
            DeviceEvent::Button { button, state, .. } => {
                self.buttons_observed = true;
                if *state == ButtonState::Pressed {
                    self.buttons.insert(*button);
                } else {
                    self.buttons.remove(button);
                }
            }
            DeviceEvent::Motion { .. } => self.motion_observed = true,
            DeviceEvent::Connected(_) | DeviceEvent::Disconnected(_) => {}
        }
    }
}
