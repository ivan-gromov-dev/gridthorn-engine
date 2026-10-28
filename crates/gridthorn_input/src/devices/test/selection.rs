use crate::{InputBuffer, InputEvent, devices::*};

#[test]
fn capabilities_are_explicit_and_selection_never_falls_back() {
    let mut input = InputBuffer::new();
    let unknown = input.snapshot();
    assert_eq!(
        unknown.input_capabilities().raw_devices,
        DeviceDiscovery::Unknown
    );
    assert_eq!(
        InputSource::Keyboard.validate(&unknown),
        Err(DeviceSelectionError::Unavailable(InputSource::Keyboard))
    );
    input.push(InputEvent::Capabilities(InputCapabilities {
        aggregate_keyboard: true,
        aggregate_mouse: true,
        raw_devices: DeviceDiscovery::Unsupported,
    }));
    let aggregate = input.snapshot();
    assert!(InputSource::Keyboard.validate(&aggregate).is_ok());
    assert!(InputSource::Mouse.validate(&aggregate).is_ok());
    let selected = InputSource::Device(InputDeviceId(1));
    assert_eq!(
        selected.validate(&aggregate),
        Err(DeviceSelectionError::Unsupported)
    );
    assert_eq!(
        input.snapshot().input_capabilities(),
        aggregate.input_capabilities()
    );
    input.push(InputEvent::Capabilities(InputCapabilities {
        raw_devices: DeviceDiscovery::Observed,
        ..aggregate.input_capabilities()
    }));
    assert_eq!(
        selected.validate(&input.snapshot()),
        Err(DeviceSelectionError::Disabled)
    );
    input.push(InputEvent::RawInputChanged {
        enabled: true,
        error: None,
    });
    assert_eq!(
        selected.validate(&input.snapshot()),
        Err(DeviceSelectionError::Unavailable(selected))
    );
    input.push(InputEvent::Device(DeviceEvent::Connected(InputDeviceId(1))));
    assert!(selected.validate(&input.snapshot()).is_ok());
    input.push(InputEvent::Device(DeviceEvent::Disconnected(
        InputDeviceId(1),
    )));
    assert_eq!(
        selected.validate(&input.snapshot()),
        Err(DeviceSelectionError::Unavailable(selected))
    );
}
