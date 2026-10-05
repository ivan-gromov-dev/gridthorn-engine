use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use gridthorn_world::{ScheduleBuilder, ScheduleStage};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::windows::EventLoopBuilderExtWindows,
};

use super::WinitApplication;
use crate::{ApplicationError, ApplicationRuntime, WindowConfig, WindowControl, WindowLifecycle};

/// Exercises opt-in enumeration and confirmed Windows monitor placement without a GPU.
#[test]
#[ignore = "requires a native Windows desktop; run this test alone"]
fn queries_and_selects_native_displays_on_request() {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let mut state = WinitApplication::new(WindowConfig::default(), DisplayProbe::default());
    state.rendering_enabled = false;
    event_loop.run_app(&mut state).unwrap();
    assert_eq!(state.lifecycle.queries, 2);
    assert!(state.lifecycle.applied);
    state.finish(Ok(())).unwrap();
}

#[derive(Default)]
struct DisplayProbe {
    queries: usize,
    frames: usize,
    target: Option<crate::display::MonitorId>,
    selected: bool,
    applied: bool,
}

impl WindowLifecycle for DisplayProbe {
    fn displays_changed(&mut self, displays: crate::display::Displays) {
        assert_eq!(
            displays.availability(),
            crate::display::DisplayAvailability::Available
        );
        assert_ne!(displays.monitors(), []);
        for monitor in displays.monitors() {
            assert!(monitor.resolution.width > 0 && monitor.resolution.height > 0);
            assert!(monitor.scale_factor.is_finite() && monitor.scale_factor > 0.0);
            assert!(monitor.modes.windows(2).all(|pair| pair[0] < pair[1]));
            println!(
                "native_display: {:?}, {:?}, modes={}",
                monitor.id,
                monitor.resolution,
                monitor.modes.len()
            );
        }
        if self.queries == 0 {
            self.target = displays
                .monitors()
                .iter()
                .find(|monitor| Some(monitor.id) != displays.active())
                .or_else(|| displays.monitors().first())
                .map(|monitor| monitor.id);
        }
        self.queries += 1;
    }

    fn monitor_selection_changed(&mut self, selection: crate::display::MonitorSelection) {
        match selection {
            crate::display::MonitorSelection::Applied { monitor } => {
                assert_eq!(Some(monitor), self.target);
                self.applied = true;
            }
            crate::display::MonitorSelection::Pending { .. } => {}
            crate::display::MonitorSelection::Failed { error, .. } => {
                panic!("native selection failed: {error}")
            }
        }
    }

    fn started(&mut self, _control: &mut WindowControl) -> Result<(), ApplicationError> {
        assert_eq!(self.queries, 0);
        Ok(())
    }

    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.frames += 1;
        if self.frames <= 5 {
            assert_eq!(self.queries, 0);
        }
        if self.frames == 5 {
            control.refresh_displays();
        }
        if !self.selected
            && let Some(monitor) = self.target
        {
            control.select_monitor(monitor);
            self.selected = true;
        }
        if self.frames >= 30 && self.applied {
            control.exit();
        }
        assert!(self.frames < 1000, "monitor placement never completed");
        control.wake_at(Instant::now() + Duration::from_millis(8));
        Ok(())
    }
}
/// Exercises actual Windows event-loop, GPU/window creation and orderly teardown.
#[test]
#[ignore = "requires a native Windows desktop and GPU; run alone in release mode"]
fn measure_native_window_shutdown() {
    assert!(
        !std::hint::black_box(cfg!(debug_assertions)),
        "run with --release"
    );
    let completed = Arc::new(AtomicUsize::new(0));
    let mut schedules = ScheduleBuilder::new();
    for _ in 0..1024 {
        let completed = completed.clone();
        schedules.add_system(ScheduleStage::Shutdown, move |_| {
            completed.fetch_add(1, Ordering::Relaxed);
        });
    }
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime.startup().unwrap();
    let start = Instant::now();
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut state = WinitApplication::new(
        WindowConfig {
            title: "Gridthorn lifecycle measurement".to_owned(),
            width: 1000,
            height: 800,
        },
        NativeLifecycle {
            runtime,
            start,
            frames: 0,
        },
    );
    event_loop.run_app(&mut state).unwrap();
    let finish = Instant::now();
    state.finish(Ok(())).unwrap();
    println!("native_window,finish_drop,{}", finish.elapsed().as_nanos());
    assert_eq!(completed.load(Ordering::Relaxed), 1024);
}

struct NativeLifecycle {
    runtime: ApplicationRuntime,
    start: Instant,
    frames: usize,
}

impl WindowLifecycle for NativeLifecycle {
    fn started(&mut self, _control: &mut WindowControl) -> Result<(), ApplicationError> {
        println!(
            "native_window,initialization,{}",
            self.start.elapsed().as_nanos()
        );
        Ok(())
    }

    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.frames += 1;
        if self.frames >= 120 {
            control.exit();
        } else {
            control.wake_at(Instant::now() + Duration::from_millis(8));
        }
        Ok(())
    }

    fn shutdown(&mut self) {
        let start = Instant::now();
        self.runtime.shutdown();
        self.runtime.shutdown();
        println!(
            "native_window,shutdown_schedule,{}",
            start.elapsed().as_nanos()
        );
    }
}
