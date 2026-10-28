use super::*;

#[test]
fn opaque_ids_distinguish_direction_and_do_not_use_display_names() {
    let output = AudioDeviceId("Output:endpoint-1".into());
    let capture = AudioDeviceId("Capture:endpoint-1".into());
    assert_ne!(output, capture);
    assert_eq!(
        AudioDeviceSelection::default(),
        AudioDeviceSelection::Default
    );
    assert_ne!(
        AudioDeviceSelection::Device(output),
        AudioDeviceSelection::Device(capture)
    );
}

#[test]
#[ignore = "requires native audio host; enumerates without opening capture"]
fn native_inventory_and_output_switch() {
    let inventory = audio_devices().unwrap();
    println!("{inventory:#?}");
    let Some(device) = inventory
        .iter()
        .find(|d| d.kind == AudioDeviceKind::Output && d.is_default)
    else {
        return;
    };
    let mut output = crate::AudioOutput::new().unwrap();
    output
        .select_device(AudioDeviceSelection::Device(device.id.clone()))
        .unwrap();
    assert_eq!(output.device(), &device.id);
    assert!(!output.refresh_device().unwrap());
    for endpoint in inventory
        .iter()
        .filter(|d| d.kind == AudioDeviceKind::Output)
    {
        output
            .select_device(AudioDeviceSelection::Device(endpoint.id.clone()))
            .unwrap();
        assert_eq!(output.device(), &endpoint.id);
    }
    output.select_device(AudioDeviceSelection::Default).unwrap();
    let mut commands = crate::AudioCommandQueue::new();
    output.process(&mut commands).unwrap();
}

#[test]
fn missing_explicit_endpoint_is_typed_and_never_falls_back() {
    let selection =
        AudioDeviceSelection::Device(AudioDeviceId("Output:gridthorn-missing-test-device".into()));
    assert!(matches!(
        resolve(&selection, AudioDeviceKind::Output),
        Err(AudioDeviceError::Unavailable(_))
    ));
}

#[test]
#[ignore = "requires native output; rejected switch must preserve working stream"]
fn rejected_switch_preserves_selection_and_stream() {
    let mut output = crate::AudioOutput::new().unwrap();
    let previous = output.device().clone();
    let missing =
        AudioDeviceSelection::Device(AudioDeviceId("Output:gridthorn-missing-test-device".into()));
    assert!(output.select_device(missing).is_err());
    assert_eq!(output.selection(), &AudioDeviceSelection::Default);
    assert_eq!(output.device(), &previous);
    assert!(!output.refresh_device().unwrap());
}

#[test]
fn transient_stream_notifications_do_not_trigger_restart_loops() {
    use kira::backend::cpal::cpal::ErrorKind;
    for kind in [
        ErrorKind::Xrun,
        ErrorKind::RealtimeDenied,
        ErrorKind::DeviceChanged,
    ] {
        assert!(!requires_reconnect(kind));
    }
    for kind in [
        ErrorKind::DeviceNotAvailable,
        ErrorKind::StreamInvalidated,
        ErrorKind::HostUnavailable,
        ErrorKind::BackendError,
    ] {
        assert!(requires_reconnect(kind));
    }
}
