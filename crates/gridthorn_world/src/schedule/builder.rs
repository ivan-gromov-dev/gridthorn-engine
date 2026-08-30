use bevy_ecs::{schedule::Schedule, world::World};

use crate::WorldAccess;

use super::{ScheduleRuntime, ScheduleStage, runtime::LifecycleSchedules};

type WorldSystem = Box<dyn for<'world> FnMut(&mut WorldAccess<'world>) + Send + Sync>;

/// Builder for explicit Gridthorn lifecycle schedules.
#[derive(Default)]
pub struct ScheduleBuilder {
    startup: Vec<WorldSystem>,
    poll_events: Vec<WorldSystem>,
    input: Vec<WorldSystem>,
    scene_transition: Vec<WorldSystem>,
    fixed_update: Vec<WorldSystem>,
    update: Vec<WorldSystem>,
    post_update: Vec<WorldSystem>,
    render: Vec<WorldSystem>,
    shutdown: Vec<WorldSystem>,
}

impl ScheduleBuilder {
    /// Create an empty schedule set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a system to one lifecycle stage.
    pub fn add_system(
        &mut self,
        stage: ScheduleStage,
        system: impl for<'world> FnMut(&mut WorldAccess<'world>) + Send + Sync + 'static,
    ) -> &mut Self {
        let systems = match stage {
            ScheduleStage::Startup => &mut self.startup,
            ScheduleStage::PollEvents => &mut self.poll_events,
            ScheduleStage::Input => &mut self.input,
            ScheduleStage::SceneTransition => &mut self.scene_transition,
            ScheduleStage::FixedUpdate => &mut self.fixed_update,
            ScheduleStage::Update => &mut self.update,
            ScheduleStage::PostUpdate => &mut self.post_update,
            ScheduleStage::Render => &mut self.render,
            ScheduleStage::Shutdown => &mut self.shutdown,
        };
        systems.push(Box::new(system));
        self
    }

    /// Build an isolated world and its three executable schedules.
    #[must_use]
    pub fn build(self) -> ScheduleRuntime {
        ScheduleRuntime::new(
            World::new(),
            LifecycleSchedules {
                startup: build_schedule(self.startup),
                poll_events: build_schedule(self.poll_events),
                input: build_schedule(self.input),
                scene_transition: build_schedule(self.scene_transition),
                fixed_update: build_schedule(self.fixed_update),
                update: build_schedule(self.update),
                post_update: build_schedule(self.post_update),
                render: build_schedule(self.render),
                shutdown: build_schedule(self.shutdown),
            },
        )
    }
}

fn build_schedule(mut systems: Vec<WorldSystem>) -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(move |backend: &mut World| {
        let mut world = WorldAccess { backend };
        for system in &mut systems {
            system(&mut world);
        }
    });
    schedule
}
