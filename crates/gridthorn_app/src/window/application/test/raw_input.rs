use super::WinitApplication;
use crate::{ApplicationError, WindowConfig, WindowControl, WindowLifecycle};
use gridthorn_input::InputEvent;
use std::time::{Duration, Instant};
use winit::{event_loop::EventLoop, platform::windows::EventLoopBuilderExtWindows};

#[test]
#[ignore = "requires a native Windows desktop; run alone"]
fn raw_observation_only_runs_between_explicit_start_and_stop() {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let mut state = WinitApplication::new(WindowConfig::default(), RawProbe::default());
    state.rendering_enabled = false;
    event_loop.run_app(&mut state).unwrap();
    assert_eq!(state.lifecycle.feedback, vec![true, false]);
    state.finish(Ok(())).unwrap();
}

#[derive(Default)]
struct RawProbe {
    frames: usize,
    feedback: Vec<bool>,
}

impl WindowLifecycle for RawProbe {
    fn input(&mut self, event: InputEvent) -> Result<(), ApplicationError> {
        match event {
            InputEvent::RawInputChanged { enabled, error } => {
                assert_eq!(error, None);
                self.feedback.push(enabled);
            }
            InputEvent::Device(_) => assert_eq!(self.feedback.last(), Some(&true)),
            _ => {}
        }
        Ok(())
    }

    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.frames += 1;
        if self.frames == 2 {
            assert_eq!(self.feedback, Vec::<bool>::new());
            control.set_raw_input(true);
        } else if self.feedback == [true] {
            control.set_raw_input(false);
        } else if self.feedback == [true, false] {
            control.exit();
        }
        assert!(self.frames < 100, "raw observation feedback did not arrive");
        control.wake_at(Instant::now() + Duration::from_millis(5));
        Ok(())
    }
}
