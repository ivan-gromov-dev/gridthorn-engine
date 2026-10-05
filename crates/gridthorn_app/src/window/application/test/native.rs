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
