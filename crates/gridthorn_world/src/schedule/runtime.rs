use bevy_ecs::{schedule::Schedule, world::World};

use crate::WorldAccess;

/// ECS world driven by explicit Gridthorn lifecycle schedules.
pub struct ScheduleRuntime {
    world: World,
    schedules: LifecycleSchedules,
    startup_complete: bool,
    shutdown_complete: bool,
}

pub(crate) struct LifecycleSchedules {
    pub(super) startup: Schedule,
    pub(super) poll_events: Schedule,
    pub(super) input: Schedule,
    pub(super) fixed_update: Schedule,
    pub(super) update: Schedule,
    pub(super) post_update: Schedule,
    pub(super) render: Schedule,
    pub(super) shutdown: Schedule,
}

impl ScheduleRuntime {
    pub(crate) fn new(world: World, schedules: LifecycleSchedules) -> Self {
        Self {
            world,
            schedules,
            startup_complete: false,
            shutdown_complete: false,
        }
    }

    /// Run the platform-event collection schedule once.
    pub fn run_poll_events(&mut self) {
        self.schedules.poll_events.run(&mut self.world);
    }

    /// Run the input-to-command mapping schedule once.
    pub fn run_input(&mut self) {
        self.schedules.input.run(&mut self.world);
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
        self.schedules.startup.run(&mut self.world);
        self.startup_complete = true;
    }

    /// Run the authoritative fixed-update schedule once.
    pub fn run_fixed_update(&mut self) {
        self.schedules.fixed_update.run(&mut self.world);
    }

    /// Run the presentation update schedule once.
    pub fn run_update(&mut self) {
        self.schedules.update.run(&mut self.world);
    }

    /// Run the presentation-finalization schedule once.
    pub fn run_post_update(&mut self) {
        self.schedules.post_update.run(&mut self.world);
    }

    /// Run the immutable-presentation consumer schedule once.
    pub fn run_render(&mut self) {
        self.schedules.render.run(&mut self.world);
    }

    /// Run `Shutdown` once and ignore subsequent requests.
    pub fn run_shutdown(&mut self) {
        if self.shutdown_complete {
            return;
        }
        self.schedules.shutdown.run(&mut self.world);
        self.shutdown_complete = true;
    }
}
