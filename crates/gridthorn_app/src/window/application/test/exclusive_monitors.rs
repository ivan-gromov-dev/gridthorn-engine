use super::WinitApplication;
use crate::{
    ApplicationError, WindowConfig, WindowControl, WindowLifecycle,
    display::{DisplayMode, Displays, MonitorInfo},
    settings::*,
};
use std::time::{Duration, Instant};
use winit::{event_loop::EventLoop, platform::windows::EventLoopBuilderExtWindows};

#[test]
#[ignore = "requires two native Windows monitors with mode switching permitted; run alone"]
fn moving_exclusive_between_monitors_restores_both_desktops() {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let mut app = WinitApplication::new(WindowConfig::default(), Probe::default());
    app.rendering_enabled = false;
    event_loop.run_app(&mut app).unwrap();
    assert!(app.lifecycle.done);
    assert_eq!(app.lifecycle.queries, 5);
    app.finish(Ok(())).unwrap();
}
#[derive(Default)]
struct Probe {
    monitors: Vec<MonitorInfo>,
    modes: Vec<DisplayMode>,
    applied_modes: Vec<DisplayMode>,
    step: u64,
    applied: u64,
    queries: usize,
    done: bool,
    started: Option<Instant>,
}
impl WindowLifecycle for Probe {
    fn started(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.started = Some(Instant::now());
        control.refresh_displays();
        Ok(())
    }
    fn displays_changed(&mut self, displays: Displays) {
        self.queries += 1;
        if self.monitors.is_empty() {
            self.monitors = displays.monitors().iter().take(2).cloned().collect();
            assert_eq!(self.monitors.len(), 2);
            self.modes = self
                .monitors
                .iter()
                .map(|monitor| {
                    monitor
                        .modes
                        .iter()
                        .find(|mode| {
                            mode.resolution == monitor.resolution
                                && mode.bit_depth == 32
                                && mode.refresh_rate_millihertz == 60_000
                        })
                        .copied()
                        .unwrap()
                })
                .collect();
        } else if self.step == 3 || self.step == 4 {
            for (index, original) in self.monitors.iter().enumerate() {
                let actual = displays
                    .monitors()
                    .iter()
                    .find(|monitor| monitor.id == original.id)
                    .unwrap();
                let expected_rate = if self.step == 3 && index == 1 {
                    Some(self.applied_modes[1].refresh_rate_millihertz)
                } else {
                    original.refresh_rate_millihertz
                };
                assert_eq!(actual.refresh_rate_millihertz, expected_rate);
                println!(
                    "exclusive_monitor_switch: step={}, monitor={}, refresh={expected_rate:?}",
                    self.step, index
                );
            }
            self.done = self.step == 4;
        }
    }
    fn window_operation_changed(&mut self, operation: WindowOperation) {
        match operation {
            WindowOperation::Pending { .. } => {}
            WindowOperation::Applied { id, state } => {
                assert_eq!(id, self.step);
                if id <= 2 {
                    let index = usize::try_from(id - 1).unwrap();
                    assert_eq!(state.mode, WindowModeKind::Exclusive);
                    assert_eq!(state.monitor, Some(self.monitors[index].id));
                    let observed = state.display_mode.unwrap();
                    assert_eq!(observed.resolution, self.modes[index].resolution);
                    assert_eq!(observed.bit_depth, self.modes[index].bit_depth);
                    self.applied_modes.push(observed);
                } else {
                    assert_eq!(state.mode, WindowModeKind::Windowed);
                }
                self.applied = id;
            }
            WindowOperation::Failed { error, state, .. } => {
                panic!("exclusive transfer failed: {error}, actual={state:?}")
            }
        }
    }
    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        assert!(self.started.unwrap().elapsed() < Duration::from_secs(30));
        if self.done {
            control.exit();
            return Ok(());
        }
        if self.monitors.len() == 2 && self.step == self.applied {
            self.step += 1;
            let mode = match self.step {
                1 | 2 => {
                    let index = usize::try_from(self.step - 1).unwrap();
                    WindowMode::Exclusive {
                        monitor: self.monitors[index].id,
                        mode: self.modes[index],
                    }
                }
                3 => {
                    control.refresh_displays();
                    WindowMode::Windowed
                }
                4 => {
                    control.refresh_displays();
                    control.wake_at(Instant::now() + Duration::from_millis(10));
                    return Ok(());
                }
                _ => unreachable!(),
            };
            control.configure_window(
                self.step,
                WindowRequest {
                    mode: Some(mode),
                    ..WindowRequest::default()
                },
            );
        }
        control.wake_at(Instant::now() + Duration::from_millis(10));
        Ok(())
    }
}
