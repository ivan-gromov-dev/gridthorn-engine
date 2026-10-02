use super::RuntimeWindowLifecycle;
use crate::{ApplicationRuntime, WindowControl, WindowLifecycle, WindowViewport};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};
use std::time::Duration;

#[derive(Default)]
struct Extents(Vec<WindowViewport>);

#[test]
fn actual_initial_resize_and_zero_extents_reach_frame_systems() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(Extents::default());
    });
    schedules.add_system(ScheduleStage::Input, |world| {
        let extent = world
            .read_resource(|v: &WindowViewport| *v)
            .expect("viewport before input");
        world.update_resource(|trace: &mut Extents| trace.0.push(extent));
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    lifecycle.resized(1280, 800);
    lifecycle
        .started(&mut WindowControl::default())
        .expect("startup");
    lifecycle
        .run_elapsed_frame(Duration::ZERO)
        .expect("initial frame");
    lifecycle.resized(1600, 1000);
    lifecycle
        .run_elapsed_frame(Duration::ZERO)
        .expect("resized frame");
    lifecycle.resized(0, 0);
    lifecycle
        .run_elapsed_frame(Duration::ZERO)
        .expect("minimized frame");
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|trace: &Extents| trace.0.clone()),
        Some(vec![
            WindowViewport {
                width: 1280,
                height: 800
            },
            WindowViewport {
                width: 1600,
                height: 1000
            },
            WindowViewport::default()
        ])
    );
}
