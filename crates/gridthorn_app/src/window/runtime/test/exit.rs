use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use crate::{ApplicationRuntime, ExitRequest, WindowControl, WindowLifecycle};

use super::RuntimeWindowLifecycle;

#[test]
fn turns_runtime_exit_resource_into_window_control() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Update, |world| {
        world.update_resource(|exit: &mut ExitRequest| exit.request());
    });
    let runtime = ApplicationRuntime::new(schedules.build());
    let mut lifecycle = RuntimeWindowLifecycle::new(runtime);
    let mut control = WindowControl::default();

    lifecycle.idle(&mut control).expect("frame should run");

    assert!(control.exit_requested);
}
