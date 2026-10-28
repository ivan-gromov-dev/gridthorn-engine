use super::{InputBuffer, InputState};
use crate::devices::{DeviceEvent, DeviceState, InputCapabilities, InputDeviceId};

impl InputState {
    /// Applied observation state, independent of the adapter's support capabilities.
    #[must_use]
    pub const fn raw_input_enabled(&self) -> bool {
        self.raw_input_enabled
    }
    /// Latest adapter capabilities; default means unreported, including headless injection.
    #[must_use]
    pub const fn input_capabilities(&self) -> InputCapabilities {
        self.capabilities
    }

    /// Observed raw connections in session-key order, not a complete hardware inventory.
    pub fn devices(&self) -> impl Iterator<Item = &DeviceState> {
        self.devices.values()
    }

    /// Find a current raw connection.
    #[must_use]
    pub fn device(&self, id: InputDeviceId) -> Option<&DeviceState> {
        self.devices.get(&id)
    }
}

impl InputBuffer {
    pub(super) fn apply_raw_input_status(&mut self, enabled: bool) {
        self.state.raw_input_enabled = enabled;
        if !enabled {
            self.state.devices.clear();
        }
    }

    pub(super) fn apply_device(&mut self, event: &DeviceEvent) {
        if !self.state.raw_input_enabled {
            return;
        }
        match event {
            DeviceEvent::Connected(id) => {
                self.state
                    .devices
                    .entry(*id)
                    .or_insert_with(|| DeviceState::new(*id));
            }
            DeviceEvent::Disconnected(id) => {
                self.state.devices.remove(id);
            }
            DeviceEvent::Key { id, .. }
            | DeviceEvent::Button { id, .. }
            | DeviceEvent::Motion { id, .. } => {
                if let Some(device) = self.state.devices.get_mut(id) {
                    device.apply(event);
                }
            }
        }
    }
}
