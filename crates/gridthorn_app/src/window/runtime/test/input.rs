use std::time::Duration;

use gridthorn_input::{ButtonState, InputEvent, InputState, KeyCode};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use crate::{ApplicationRuntime, WindowLifecycle};

use super::RuntimeWindowLifecycle;

#[derive(Default)]
struct InputTrace(Vec<(bool, bool)>);

#[test]
fn publishes_frame_input_before_the_input_schedule() {
    let mut schedules = ScheduleBuilder::new();
    schedules
        .add_system(ScheduleStage::Startup, |world| {
            world.insert_resource(InputTrace::default());
        })
        .add_system(ScheduleStage::Input, |world| {
            let state = world
                .read_resource(|input: &InputState| input.clone())
                .expect("window runtime should publish input before Input");
            world.update_resource(|trace: &mut InputTrace| {
                trace.0.push((
                    state.key_down(KeyCode::KeyD),
                    state.key_just_pressed(KeyCode::KeyD),
                ));
            });
        });
    let runtime = ApplicationRuntime::new(schedules.build());
    let mut lifecycle = RuntimeWindowLifecycle::new(runtime);
    lifecycle
        .input(InputEvent::Keyboard {
            key: KeyCode::KeyD,
            state: ButtonState::Pressed,
        })
        .expect("input should be accepted");

    lifecycle
        .run_elapsed_frame(Duration::ZERO)
        .expect("first frame should run");
    lifecycle
        .run_elapsed_frame(Duration::ZERO)
        .expect("second frame should run");

    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|trace: &InputTrace| trace.0.clone()),
        Some(vec![(true, true), (true, false)])
    );
}
