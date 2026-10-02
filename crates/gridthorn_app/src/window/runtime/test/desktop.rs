use std::time::Duration;

use super::super::RuntimeWindowLifecycle;
use crate::{ApplicationRuntime, WindowControl, WindowLifecycle};
use gridthorn_input::{
    ButtonState, InputEvent, InputState, KeyCode, PointerCapture, PointerCaptureMode,
};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

#[test]
fn runtime_delivers_capture_requests_once_and_cancels_held_input_on_suspend() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        assert!(world.update_resource(|capture: &mut PointerCapture| {
            capture.request(PointerCaptureMode::Confined);
        }));
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    lifecycle.started(&mut WindowControl::default()).unwrap();
    let mut control = WindowControl::default();
    lifecycle.idle(&mut control).unwrap();
    assert_eq!(control.capture, Some(PointerCaptureMode::Confined));
    let mut next = WindowControl::default();
    lifecycle.idle(&mut next).unwrap();
    assert_eq!(next.capture, None);
    lifecycle
        .input(InputEvent::Keyboard {
            key: KeyCode::F24,
            state: ButtonState::Pressed,
        })
        .unwrap();
    lifecycle.run_elapsed_frame(Duration::ZERO).unwrap();
    lifecycle.suspended();
    lifecycle.resumed();
    lifecycle.run_elapsed_frame(Duration::ZERO).unwrap();
    lifecycle
        .runtime
        .world()
        .read_resource(|input: &InputState| {
            assert!(!input.key_down(KeyCode::F24));
            assert!(input.key_just_released(KeyCode::F24));
            assert_eq!(input.events(), [InputEvent::FocusLost]);
        })
        .unwrap();
}
