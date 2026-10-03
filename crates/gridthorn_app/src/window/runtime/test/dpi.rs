use super::RuntimeWindowLifecycle;
use crate::{ApplicationRuntime, WindowControl, WindowLifecycle, WindowScaleFactor};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};
use std::time::Duration;

#[test]
fn initial_and_changed_dpi_reach_startup_and_frame_systems() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        assert_eq!(
            world.read_resource(|dpi: &WindowScaleFactor| dpi.0),
            Some(1.25)
        );
    });
    schedules.add_system(ScheduleStage::Update, |world| {
        assert_eq!(
            world.read_resource(|dpi: &WindowScaleFactor| dpi.0),
            Some(2.0)
        );
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    lifecycle.scale_factor_changed(1.25);
    lifecycle
        .started(&mut WindowControl::default())
        .expect("startup");
    lifecycle.scale_factor_changed(2.0);
    lifecycle
        .run_elapsed_frame(Duration::ZERO)
        .expect("DPI changed frame");
}
