use bevy_ecs::{schedule::Schedule, world::World};

use crate::WorldAccess;

use super::{ScheduleRuntime, ScheduleStage};

type WorldSystem = Box<dyn for<'world> FnMut(&mut WorldAccess<'world>) + Send + Sync>;

/// Builder for explicit Gridthorn lifecycle schedules.
#[derive(Default)]
pub struct ScheduleBuilder {
    startup: Vec<WorldSystem>,
    fixed_update: Vec<WorldSystem>,
    update: Vec<WorldSystem>,
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
            ScheduleStage::FixedUpdate => &mut self.fixed_update,
            ScheduleStage::Update => &mut self.update,
        };
        systems.push(Box::new(system));
        self
    }

    /// Build an isolated world and its three executable schedules.
    #[must_use]
    pub fn build(self) -> ScheduleRuntime {
        ScheduleRuntime::new(
            World::new(),
            build_schedule(self.startup),
            build_schedule(self.fixed_update),
            build_schedule(self.update),
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
