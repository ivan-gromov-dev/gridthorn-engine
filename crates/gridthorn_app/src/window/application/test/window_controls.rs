use super::WinitApplication;
use crate::{
    ApplicationError, WindowConfig, WindowControl, WindowLifecycle,
    display::{DisplayMode, DisplayResolution, Displays, MonitorId},
    settings::*,
};
use std::time::{Duration, Instant};
use winit::{event_loop::EventLoop, platform::windows::EventLoopBuilderExtWindows};

#[test]
#[ignore = "requires native Windows desktop; run alone"]
fn confirms_window_controls_and_restores_geometry() {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let mut app = WinitApplication::new(WindowConfig::default(), Probe::default());
    app.rendering_enabled = false;
    event_loop.run_app(&mut app).unwrap();
    assert_eq!(app.lifecycle.completed, 6);
    assert_eq!(app.lifecycle.queries, 6);
    assert!(app.lifecycle.exclusive_confirmed);
    app.finish(Ok(())).unwrap();
}
#[derive(Default)]
struct Probe {
    target: Option<(MonitorId, DisplayMode)>,
    queries: usize,
    sent: u64,
    completed: u64,
    restored: Option<WindowState>,
    started: Option<Instant>,
    desktop: Option<DisplayMode>,
    exclusive_confirmed: bool,
    applied_mode: Option<DisplayMode>,
}
impl WindowLifecycle for Probe {
    fn displays_changed(&mut self, displays: Displays) {
        self.queries += 1;
        if self.target.is_none() {
            let monitor = displays
                .monitors()
                .iter()
                .find(|monitor| Some(monitor.id) == displays.active())
                .unwrap();
            let mode = monitor
                .modes
                .iter()
                .find(|mode| {
                    mode.resolution == monitor.resolution
                        && Some(mode.refresh_rate_millihertz) == monitor.refresh_rate_millihertz
                })
                .unwrap();
            self.desktop = Some(*mode);
            let alternate = monitor
                .modes
                .iter()
                .find(|candidate| {
                    candidate.resolution == mode.resolution
                        && candidate.bit_depth == mode.bit_depth
                        && candidate.refresh_rate_millihertz == 60_000
                        && candidate.refresh_rate_millihertz != mode.refresh_rate_millihertz
                })
                .or_else(|| {
                    monitor.modes.iter().find(|candidate| {
                        candidate.resolution == mode.resolution
                            && candidate.bit_depth == mode.bit_depth
                            && candidate.refresh_rate_millihertz > 0
                            && candidate.refresh_rate_millihertz != mode.refresh_rate_millihertz
                    })
                })
                .expect(
                    "native refresh acceptance requires a second rate at the desktop resolution",
                );
            self.target = Some((monitor.id, *alternate));
        } else if let Some((monitor_id, _)) = self.target
            && (self.sent == 4 || self.sent == 6)
        {
            let monitor = displays
                .monitors()
                .iter()
                .find(|monitor| monitor.id == monitor_id)
                .unwrap();
            let expected = if self.sent == 4 {
                self.applied_mode.expect("successful exclusive readback")
            } else {
                self.desktop.unwrap()
            };
            assert_eq!(
                monitor.refresh_rate_millihertz,
                Some(expected.refresh_rate_millihertz)
            );
            assert_eq!(monitor.resolution, expected.resolution);
            println!(
                "native_fullscreen: stage={}, desktop_readback={:?}",
                self.sent, monitor.refresh_rate_millihertz
            );
        }
    }
    fn window_state_changed(&mut self, _state: WindowState, capabilities: WindowCapabilities) {
        assert!(capabilities.exclusive);
    }
    fn started(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.started = Some(Instant::now());
        control.refresh_displays();
        Ok(())
    }
    fn window_operation_changed(&mut self, operation: WindowOperation) {
        match operation {
            WindowOperation::Pending { .. } => {}
            WindowOperation::Applied { id, state } => {
                assert_eq!(id, self.sent);
                match id {
                    1 => {
                        assert_eq!(
                            state.size,
                            DisplayResolution {
                                width: 900,
                                height: 650
                            }
                        );
                        assert_eq!(state.resizable, Some(false));
                        self.restored = Some(state);
                    }
                    2 => assert_eq!(state.mode, WindowModeKind::Borderless),
                    3 => {
                        assert_eq!(state.mode, WindowModeKind::Exclusive);
                        let observed = state.display_mode.unwrap();
                        let requested = self.target.unwrap().1;
                        assert_eq!(observed.resolution, requested.resolution);
                        assert_eq!(observed.bit_depth, requested.bit_depth);
                        self.applied_mode = Some(observed);
                        self.exclusive_confirmed = true;
                    }
                    4 => {
                        let previous = self.restored.unwrap();
                        assert_eq!(state.size, previous.size);
                        assert_eq!(state.position, previous.position);
                        assert_eq!(state.resizable, Some(false));
                    }
                    5 => assert_eq!(state.resizable, Some(true)),
                    _ => panic!("unexpected successful operation"),
                }
                self.completed = id;
            }
            WindowOperation::Failed {
                id: 6,
                error: WindowOperationError::ModeUnavailable { .. },
                state,
            } => {
                assert_eq!(state.mode, WindowModeKind::Windowed);
                self.completed = 6;
            }
            WindowOperation::Failed { error, state, .. } => {
                panic!("window operation failed: {error}, actual={state:?}")
            }
        }
    }
    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        assert!(self.started.unwrap().elapsed() < Duration::from_secs(30));
        if self.completed == 6 {
            control.exit();
            return Ok(());
        }
        if self.sent == self.completed
            && let Some((monitor, mode)) = self.target
        {
            let request = match self.sent {
                0 => WindowRequest {
                    size: Some(DisplayResolution {
                        width: 900,
                        height: 650,
                    }),
                    placement: Some(WindowPlacement::Centered { monitor }),
                    resize_policy: Some(WindowResizePolicy::Fixed),
                    ..WindowRequest::default()
                },
                1 => WindowRequest {
                    mode: Some(WindowMode::Borderless { monitor }),
                    ..WindowRequest::default()
                },
                2 => WindowRequest {
                    mode: Some(WindowMode::Exclusive { monitor, mode }),
                    ..WindowRequest::default()
                },
                3 => WindowRequest {
                    mode: Some(WindowMode::Windowed),
                    ..WindowRequest::default()
                },
                4 => WindowRequest {
                    resize_policy: Some(WindowResizePolicy::Resizable {
                        min: Some(DisplayResolution {
                            width: 640,
                            height: 480,
                        }),
                        max: Some(DisplayResolution {
                            width: 1400,
                            height: 900,
                        }),
                    }),
                    ..WindowRequest::default()
                },
                5 => WindowRequest {
                    mode: Some(WindowMode::Exclusive {
                        monitor,
                        mode: DisplayMode {
                            refresh_rate_millihertz: 1,
                            ..mode
                        },
                    }),
                    ..WindowRequest::default()
                },
                _ => unreachable!(),
            };
            self.sent += 1;
            if self.sent == 4 {
                control.refresh_displays();
            }
            control.configure_window(self.sent, request);
        }
        control.wake_at(Instant::now() + Duration::from_millis(10));
        Ok(())
    }
}
