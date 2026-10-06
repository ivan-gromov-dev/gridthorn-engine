use super::WinitApplication;
use crate::presentation::*;
use crate::{ApplicationError, WindowConfig, WindowControl, WindowLifecycle};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    platform::windows::EventLoopBuilderExtWindows,
    window::WindowId,
};

struct Probe {
    start: Instant,
    idle_calls: usize,
    feedback: Option<PresentationOperation>,
}
impl WindowLifecycle for Probe {
    fn started(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        control.configure_presentation(
            1,
            PresentationConfig {
                present_mode: PresentMode::Fifo,
                frame_rate_limit: Some(FrameRateLimit::new(30).unwrap()),
            },
        );
        self.start = Instant::now();
        Ok(())
    }
    fn presentation_operation_changed(&mut self, operation: PresentationOperation) {
        self.feedback = Some(operation);
    }
    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.idle_calls += 1;
        if self.start.elapsed() > Duration::from_millis(650) {
            control.exit();
        }
        control.wake_at(Instant::now() + Duration::from_millis(1));
        Ok(())
    }
}
struct Runner {
    app: WinitApplication<Probe>,
    redraws: Vec<Instant>,
}
impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.app.resumed(event_loop);
        self.app.suspended(event_loop);
        self.app.about_to_wait(event_loop);
        assert_eq!(self.app.lifecycle.idle_calls, 0);
        let id = self.app.window.as_ref().unwrap().id();
        self.app
            .window_event(event_loop, id, WindowEvent::RedrawRequested);
        assert_eq!(
            self.app.renderer.as_ref().unwrap().applied_present_mode(),
            None
        );
        self.app.resumed(event_loop);
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.app.about_to_wait(event_loop);
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if matches!(event, WindowEvent::RedrawRequested)
            && self.app.presentation.pacer.ready(Instant::now())
        {
            self.redraws.push(Instant::now());
        }
        self.app.window_event(event_loop, id, event);
    }
}

/// Real surface mode switching, rejected modes, deferred configuration and storm-resistant pacing.
#[test]
#[ignore = "requires a native Windows desktop and GPU; run this test alone"]
fn switches_supported_native_modes_and_caps_redraws() {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let probe = Probe {
        start: Instant::now(),
        idle_calls: 0,
        feedback: None,
    };
    let mut runner = Runner {
        app: WinitApplication::new(WindowConfig::default(), probe),
        redraws: Vec::new(),
    };
    event_loop.run_app(&mut runner).unwrap();
    assert!(runner.app.error.is_none(), "{:?}", runner.app.error);
    assert!(matches!(
        runner.app.lifecycle.feedback,
        Some(PresentationOperation::Applied { id: 1, .. })
    ));
    assert!(
        runner.redraws.len() >= 3,
        "no sustained native presentation"
    );
    let interval = FrameRateLimit::new(30).unwrap().interval();
    for pair in runner.redraws.windows(2) {
        assert!(
            pair[1].duration_since(pair[0]) >= interval.saturating_sub(Duration::from_micros(100))
        );
    }
    println!(
        "presentation_pacing: {} redraws over {:?}, cap=30",
        runner.redraws.len(),
        runner
            .redraws
            .last()
            .unwrap()
            .duration_since(runner.redraws[0])
    );
    let renderer = runner.app.renderer.as_mut().unwrap();
    let modes = renderer.present_modes().to_vec();
    println!("native_present_modes: {modes:?}");
    for mode in &modes {
        renderer.set_present_mode(*mode).unwrap();
        renderer.render().unwrap();
        assert_eq!(renderer.applied_present_mode(), Some(*mode));
        renderer.resize(0, 0).unwrap();
        assert_eq!(renderer.applied_present_mode(), None);
        renderer.render().unwrap();
        renderer.resize(960, 540).unwrap();
        renderer.set_occluded(true);
        renderer.render().unwrap();
        assert_eq!(renderer.applied_present_mode(), None);
        renderer.set_occluded(false);
        renderer.render().unwrap();
        assert_eq!(renderer.applied_present_mode(), Some(*mode));
    }
    for mode in [
        PresentMode::Fifo,
        PresentMode::FifoRelaxed,
        PresentMode::Immediate,
        PresentMode::Mailbox,
    ] {
        if !modes.contains(&mode) {
            let before = renderer.applied_present_mode();
            assert!(matches!(
                renderer.set_present_mode(mode),
                Err(gridthorn_render::RenderSurfaceError::UnsupportedPresentMode { .. })
            ));
            renderer.render().unwrap();
            assert_eq!(renderer.applied_present_mode(), before);
        }
    }
    runner
        .app
        .configure_presentation(2, PresentationConfig::default());
    assert_eq!(runner.app.presentation.pacer.limit, None);
    runner.app.renderer.as_mut().unwrap().render().unwrap();
    runner.app.publish_presentation();
    assert!(matches!(
        runner.app.lifecycle.feedback,
        Some(PresentationOperation::Applied { id: 2, .. })
    ));
    runner.app.finish(Ok(())).unwrap();
}
