use std::collections::BTreeMap;

use gridthorn_input::{
    InputEvent,
    devices::{DeviceDiscovery, DeviceEvent, InputCapabilities, InputDeviceId},
};
use winit::event::{DeviceEvent as NativeEvent, DeviceId};

/// Maps backend connections into monotonic session identities without exposing native handles.
#[derive(Default)]
pub(crate) struct NativeInputDevices {
    enabled: bool,
    connections: BTreeMap<DeviceId, InputDeviceId>,
    next_id: u64,
}

impl NativeInputDevices {
    pub(crate) fn set_enabled(&mut self, enabled: bool) -> InputEvent {
        if enabled && Self::capabilities().raw_devices != DeviceDiscovery::Observed {
            return InputEvent::RawInputChanged {
                enabled: false,
                error: Some(gridthorn_input::devices::DeviceSelectionError::Unsupported),
            };
        }
        self.enabled = enabled;
        if !enabled {
            self.connections.clear();
        }
        InputEvent::RawInputChanged {
            enabled,
            error: None,
        }
    }

    pub(crate) const fn capabilities() -> InputCapabilities {
        InputCapabilities {
            aggregate_keyboard: true,
            aggregate_mouse: true,
            raw_devices: if cfg!(target_os = "windows") {
                DeviceDiscovery::Observed
            } else {
                DeviceDiscovery::Unsupported
            },
        }
    }

    pub(crate) fn map(
        &mut self,
        native: DeviceId,
        event: &NativeEvent,
        focused: bool,
    ) -> Vec<InputEvent> {
        if !self.enabled {
            return Vec::new();
        }
        if matches!(event, NativeEvent::Removed) {
            return self
                .connections
                .remove(&native)
                .map(|id| InputEvent::Device(DeviceEvent::Disconnected(id)))
                .into_iter()
                .collect();
        }
        if !matches!(event, NativeEvent::Added)
            && (!focused
                || !matches!(
                    event,
                    NativeEvent::Key(_)
                        | NativeEvent::Button { .. }
                        | NativeEvent::MouseMotion { .. }
                ))
        {
            return Vec::new();
        }
        let mut output = Vec::new();
        let id = if let Some(id) = self.connections.get(&native) {
            *id
        } else {
            let Some(next) = self.next_id.checked_add(1) else {
                tracing::error!(
                    component = "input",
                    event = "device_identity_exhausted",
                    "cannot allocate another raw input connection identity"
                );
                return output;
            };
            self.next_id = next;
            let id = InputDeviceId(next);
            self.connections.insert(native, id);
            output.push(InputEvent::Device(DeviceEvent::Connected(id)));
            id
        };
        let mapped = match event {
            NativeEvent::Key(event) => Some(DeviceEvent::Key {
                id,
                key: super::super::input::map_physical_key(event.physical_key),
                state: super::super::input::map_button_state(event.state),
            }),
            NativeEvent::Button { button, state } => Some(DeviceEvent::Button {
                id,
                button: *button,
                state: super::super::input::map_button_state(*state),
            }),
            NativeEvent::MouseMotion { delta } => Some(DeviceEvent::Motion {
                id,
                x: delta.0,
                y: delta.1,
            }),
            _ => None,
        };
        output.extend(mapped.map(InputEvent::Device));
        output
    }
}
