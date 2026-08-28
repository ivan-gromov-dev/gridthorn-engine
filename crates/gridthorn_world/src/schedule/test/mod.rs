use super::{ScheduleBuilder, ScheduleStage};

#[derive(Default)]
struct StageTrace(Vec<&'static str>);

#[test]
fn runs_startup_once_before_explicit_fixed_and_frame_updates() {
    let mut builder = ScheduleBuilder::new();
    builder
        .add_system(ScheduleStage::Startup, |world| {
            world.insert_resource(StageTrace(vec!["startup"]));
        })
        .add_system(ScheduleStage::FixedUpdate, |world| {
            world.update_resource(|trace: &mut StageTrace| trace.0.push("fixed"));
        })
        .add_system(ScheduleStage::Update, |world| {
            world.update_resource(|trace: &mut StageTrace| trace.0.push("update"));
        });
    let mut runtime = builder.build();

    runtime.run_startup();
    runtime.run_startup();
    runtime.run_fixed_update();
    runtime.run_fixed_update();
    runtime.run_update();

    let trace = runtime
        .world()
        .read_resource(|trace: &StageTrace| trace.0.clone());
    assert_eq!(trace, Some(vec!["startup", "fixed", "fixed", "update"]));
}

#[test]
fn preserves_registration_order_within_a_stage() {
    let mut builder = ScheduleBuilder::new();
    builder
        .add_system(ScheduleStage::Startup, |world| {
            world.insert_resource(StageTrace::default());
        })
        .add_system(ScheduleStage::Update, |world| {
            world.update_resource(|trace: &mut StageTrace| trace.0.push("first"));
        })
        .add_system(ScheduleStage::Update, |world| {
            world.update_resource(|trace: &mut StageTrace| trace.0.push("second"));
        });
    let mut runtime = builder.build();

    runtime.run_startup();
    runtime.run_update();

    let trace = runtime
        .world()
        .read_resource(|trace: &StageTrace| trace.0.clone());
    assert_eq!(trace, Some(vec!["first", "second"]));
}
