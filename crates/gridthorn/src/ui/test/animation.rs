use super::super::*;
use crate::{
    ApplicationRuntime, FrameTiming, ScheduleBuilder, ScheduleStage, SimulationControl,
    SimulationSpeed,
};
use std::time::Duration;

#[test]
fn ui_transition_uses_host_time_through_pause_speed_changes_and_rendering() {
    struct Interface {
        tree: UiTree,
        transition: UiTransition,
    }
    let mut node = UiNode::new(UiNodeId(0), UiControl::Button("Animate".into()));
    node.style.size = [UiLength::Pixels(80.0), UiLength::Pixels(30.0)];
    let mut interface = Some(Interface {
        tree: UiTree::new(node, UiTheme::default()).unwrap(),
        transition: UiTransition::new(
            UiNodeId(0),
            UiProperty::Offset([0.0; 2]),
            UiProperty::Offset([100.0, 0.0]),
            Duration::from_secs(1),
            UiEasing::Linear,
        )
        .unwrap(),
    });
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        world.insert_resource(interface.take().unwrap());
    });
    schedules.add_system(ScheduleStage::Update, |world| {
        let delta = world
            .read_resource(|timing: &FrameTiming| timing.frame_elapsed())
            .unwrap();
        world.update_resource(|ui: &mut Interface| {
            ui.transition.advance(&mut ui.tree, delta).unwrap();
        });
    });
    schedules.add_system(ScheduleStage::Render, |world| {
        let primitives = world
            .read_resource(|ui: &Interface| {
                ui.tree
                    .layout([200.0; 2], 2.0, None)
                    .unwrap()
                    .into_primitives()
            })
            .unwrap();
        world.insert_resource(crate::RenderFrame::default().with_ui(primitives));
    });
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime.run_timed_frame(Duration::ZERO).unwrap();
    runtime.world().update_resource(SimulationControl::pause);
    let paused = runtime.run_timed_frame(Duration::from_millis(250)).unwrap();
    assert_eq!(paused.fixed_steps(), 0);
    assert_eq!(
        runtime
            .world()
            .read_resource(|ui: &Interface| ui.tree.root().style.offset),
        Some([25.0, 0.0])
    );
    runtime
        .world()
        .update_resource(|control: &mut SimulationControl| {
            control.resume();
            control.set_speed(SimulationSpeed::new(4, 1).unwrap());
        });
    runtime.run_timed_frame(Duration::from_millis(250)).unwrap();
    assert_eq!(
        runtime
            .world()
            .read_resource(|ui: &Interface| ui.tree.root().style.offset),
        Some([50.0, 0.0])
    );
    runtime.world().update_resource(SimulationControl::pause);
    runtime.run_timed_frame(Duration::from_millis(500)).unwrap();
    assert_eq!(
        runtime
            .world()
            .read_resource(|ui: &Interface| ui.transition.is_finished()),
        Some(true)
    );
    assert!(
        runtime
            .world()
            .read_resource(|frame: &crate::RenderFrame| !frame.ui().is_empty())
            .unwrap()
    );
}
