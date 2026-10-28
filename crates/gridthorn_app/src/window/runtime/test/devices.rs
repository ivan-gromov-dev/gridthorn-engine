use super::RuntimeWindowLifecycle;
use crate::{ApplicationRuntime, WindowControl, WindowLifecycle};
use gridthorn_input::{
    InputEvent, InputState,
    devices::{DeviceDiscovery, DeviceEvent, InputCapabilities, InputDeviceId},
};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};
use std::time::Duration;

#[test]
fn raw_requests_are_installed_before_startup_and_never_implicitly_repeated() {
    use gridthorn_input::devices::RawInput;
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        assert_eq!(
            world.read_resource(InputState::raw_input_enabled),
            Some(false)
        );
        world.update_resource(|raw: &mut RawInput| {
            raw.request(true);
            raw.request(false);
            raw.request(true);
        });
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    let mut startup = WindowControl::default();
    lifecycle.started(&mut startup).unwrap();
    assert_eq!(startup.raw_input, Some(true));
    let mut idle = WindowControl::default();
    lifecycle.idle(&mut idle).unwrap();
    assert_eq!(idle.raw_input, None);
    lifecycle
        .input(InputEvent::RawInputChanged {
            enabled: true,
            error: None,
        })
        .unwrap();
    lifecycle.run_elapsed_frame(Duration::ZERO).unwrap();
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(InputState::raw_input_enabled),
        Some(true)
    );
    lifecycle
        .runtime
        .world()
        .update_resource(|raw: &mut RawInput| raw.request(false));
    let mut stop = WindowControl::default();
    lifecycle.idle(&mut stop).unwrap();
    assert_eq!(stop.raw_input, Some(false));
    lifecycle
        .input(InputEvent::RawInputChanged {
            enabled: false,
            error: None,
        })
        .unwrap();
    lifecycle.run_elapsed_frame(Duration::ZERO).unwrap();
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(InputState::raw_input_enabled),
        Some(false)
    );
}

#[test]
fn publishes_adapter_capabilities_before_startup_and_inventory_before_input() {
    let capabilities = InputCapabilities {
        aggregate_keyboard: true,
        aggregate_mouse: true,
        raw_devices: DeviceDiscovery::Observed,
    };
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        assert_eq!(
            world.read_resource(|input: &InputState| input.input_capabilities()),
            Some(capabilities)
        );
    });
    schedules.add_system(ScheduleStage::Input, |world| {
        assert_eq!(
            world.read_resource(|input: &InputState| input.devices().count()),
            Some(1)
        );
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    lifecycle
        .input(InputEvent::Capabilities(capabilities))
        .unwrap();
    lifecycle.started(&mut WindowControl::default()).unwrap();
    lifecycle
        .input(InputEvent::RawInputChanged {
            enabled: true,
            error: None,
        })
        .unwrap();
    lifecycle
        .input(InputEvent::Device(DeviceEvent::Connected(InputDeviceId(1))))
        .unwrap();
    lifecycle.run_elapsed_frame(Duration::ZERO).unwrap();
}
