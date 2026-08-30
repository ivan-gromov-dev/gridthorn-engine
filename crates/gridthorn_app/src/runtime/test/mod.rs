use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use super::{ApplicationRuntime, LifecycleError};

#[derive(Default)]
struct StageTrace(Vec<&'static str>);

#[test]
fn runs_complete_frame_order_with_requested_fixed_steps() {
    let mut schedules = ScheduleBuilder::new();
    for (stage, name) in [
        (ScheduleStage::Startup, "startup"),
        (ScheduleStage::PollEvents, "poll_events"),
        (ScheduleStage::Input, "input"),
        (ScheduleStage::FixedUpdate, "fixed_update"),
        (ScheduleStage::Update, "update"),
        (ScheduleStage::PostUpdate, "post_update"),
        (ScheduleStage::Render, "render"),
        (ScheduleStage::Shutdown, "shutdown"),
    ] {
        schedules.add_system(stage, move |world| {
            if stage == ScheduleStage::Startup {
                world.insert_resource(StageTrace::default());
            }
            world.update_resource(|trace: &mut StageTrace| trace.0.push(name));
        });
    }
    let mut runtime = ApplicationRuntime::new(schedules.build());

    runtime.run_frame(2).expect("first frame should run");
    runtime.run_frame(0).expect("second frame should run");
    runtime.shutdown();
    runtime.shutdown();

    let trace = runtime
        .world()
        .read_resource(|trace: &StageTrace| trace.0.clone());
    assert_eq!(
        trace,
        Some(vec![
            "startup",
            "poll_events",
            "input",
            "fixed_update",
            "fixed_update",
            "update",
            "post_update",
            "render",
            "poll_events",
            "input",
            "update",
            "post_update",
            "render",
            "shutdown",
        ])
    );
}

#[test]
fn rejects_frames_after_shutdown() {
    let schedules = ScheduleBuilder::new();
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime.shutdown();

    assert_eq!(runtime.run_frame(0), Err(LifecycleError::AlreadyShutdown));
}
