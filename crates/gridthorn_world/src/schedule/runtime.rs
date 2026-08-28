use bevy_ecs::{schedule::Schedule, world::World};

use crate::WorldAccess;

/// ECS world driven by explicit Gridthorn lifecycle schedules.
pub struct ScheduleRuntime {
    world: World,
    startup: Schedule,
    fixed_update: Schedule,
    update: Schedule,
    startup_complete: bool,
}

impl ScheduleRuntime {
    pub(crate) fn new(
        world: World,
        startup: Schedule,
        fixed_update: Schedule,
        update: Schedule,
    ) -> Self {
        Self {
            world,
            startup,
            fixed_update,
            update,
            startup_complete: false,
        }
    }

    /// Borrow world state for setup or inspection between schedule runs.
    pub fn world(&mut self) -> WorldAccess<'_> {
        WorldAccess {
            backend: &mut self.world,
        }
    }

    /// Run `Startup` once and ignore subsequent requests.
    pub fn run_startup(&mut self) {
        if self.startup_complete {
            return;
        }
        self.startup.run(&mut self.world);
        self.startup_complete = true;
    }

    /// Run the authoritative fixed-update schedule once.
    pub fn run_fixed_update(&mut self) {
        self.fixed_update.run(&mut self.world);
    }

    /// Run the presentation update schedule once.
    pub fn run_update(&mut self) {
        self.update.run(&mut self.world);
    }
}
