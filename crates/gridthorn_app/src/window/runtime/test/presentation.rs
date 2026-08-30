use std::time::Duration;

use gridthorn_render::{Camera2d, Color, RenderFrame, Sprite};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use crate::{ApplicationRuntime, WindowLifecycle};

use super::RuntimeWindowLifecycle;

#[test]
fn extracts_render_stage_snapshot_for_the_window_renderer() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Render, |world| {
        world.insert_resource(RenderFrame::new(
            Camera2d::default(),
            vec![Sprite::new(
                [4.0, 5.0],
                [16.0, 16.0],
                Color::rgb(0.2, 0.8, 0.4),
            )],
        ));
    });
    let runtime = ApplicationRuntime::new(schedules.build());
    let mut lifecycle = RuntimeWindowLifecycle::new(runtime);

    lifecycle
        .run_elapsed_frame(Duration::ZERO)
        .expect("presentation frame should run");
    let frame = lifecycle.render_frame();

    assert_eq!(frame.sprites().len(), 1);
    let position = frame.sprites()[0].position();
    assert!((position[0] - 4.0).abs() < f32::EPSILON);
    assert!((position[1] - 5.0).abs() < f32::EPSILON);
}
