use super::NativeInputDevices;
use gridthorn_input::devices::DeviceDiscovery;

#[test]
fn aggregate_support_is_explicit_and_raw_support_is_backend_scoped() {
    let capabilities = NativeInputDevices::capabilities();
    assert!(capabilities.aggregate_keyboard && capabilities.aggregate_mouse);
    assert_eq!(
        capabilities.raw_devices,
        if cfg!(target_os = "windows") {
            DeviceDiscovery::Observed
        } else {
            DeviceDiscovery::Unsupported
        }
    );
}

#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(target_os = "windows"))]
#[test]
fn unsupported_adapter_does_not_invent_raw_connections() {
    let mut devices = NativeInputDevices::default();
    assert_eq!(
        devices.set_enabled(true),
        gridthorn_input::InputEvent::RawInputChanged {
            enabled: false,
            error: Some(gridthorn_input::devices::DeviceSelectionError::Unsupported)
        }
    );
    assert_eq!(
        devices.map(
            winit::event::DeviceId::dummy(),
            &winit::event::DeviceEvent::Added,
            true
        ),
        []
    );
}
