use super::{
    WindowCapabilities, WindowMode, WindowModeKind, WindowOperation, WindowOperationError,
    WindowPlacement, WindowRequest, WindowResizePolicy, WindowState,
    pending::{ExpectedWindow, PendingWindow},
    validation::validate_constraints,
};
use crate::display::{
    DisplayMode, DisplayResolution, native::NativeDisplays, selection::centered_origin,
};
use std::time::Instant;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event_loop::ActiveEventLoop,
    window::{Fullscreen, Window},
};

#[derive(Default)]
pub(crate) struct NativeWindowSettings {
    capabilities: WindowCapabilities,
    policy: WindowResizePolicy,
    restore: Option<(DisplayResolution, Option<(i32, i32)>)>,
    pending: Option<PendingWindow>,
}
enum FullscreenChange {
    Preserve,
    Set(Option<Fullscreen>),
}
struct PreparedWindow {
    fullscreen: FullscreenChange,
    policy: Option<WindowResizePolicy>,
    expected: ExpectedWindow,
}
impl NativeWindowSettings {
    pub(crate) fn initialize(&mut self, event_loop: &ActiveEventLoop, window: &Window) {
        let desktop = cfg!(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux"
        ));
        #[cfg(target_os = "linux")]
        let wayland = {
            use winit::platform::wayland::ActiveEventLoopExtWayland;
            event_loop.is_wayland()
        };
        #[cfg(not(target_os = "linux"))]
        let wayland = {
            let _ = event_loop;
            false
        };
        self.capabilities = WindowCapabilities {
            available: desktop,
            size: desktop,
            resize_policy: desktop,
            resize_policy_feedback: desktop && (wayland || !cfg!(target_os = "linux")),
            placement: desktop && window.outer_position().is_ok(),
            borderless: desktop,
            exclusive: desktop && !wayland,
        };
    }
    pub(crate) fn capabilities(&self) -> WindowCapabilities {
        self.capabilities
    }
    pub(crate) fn state(&self, window: &Window, displays: &NativeDisplays) -> WindowState {
        let size = window.inner_size();
        let monitor = window.current_monitor();
        let (mode, display_mode) = match window.fullscreen() {
            None => (WindowModeKind::Windowed, None),
            Some(Fullscreen::Borderless(_)) => (WindowModeKind::Borderless, None),
            Some(Fullscreen::Exclusive(mode)) => {
                let size = mode.size();
                #[cfg(target_os = "windows")]
                let refresh = monitor
                    .as_ref()
                    .and_then(winit::monitor::MonitorHandle::refresh_rate_millihertz)
                    .unwrap_or(0);
                #[cfg(not(target_os = "windows"))]
                let refresh = mode.refresh_rate_millihertz();
                (
                    WindowModeKind::Exclusive,
                    Some(DisplayMode {
                        resolution: DisplayResolution {
                            width: size.width,
                            height: size.height,
                        },
                        refresh_rate_millihertz: refresh,
                        bit_depth: mode.bit_depth(),
                    }),
                )
            }
        };
        WindowState {
            size: DisplayResolution {
                width: size.width,
                height: size.height,
            },
            position: window
                .outer_position()
                .ok()
                .map(|position| (position.x, position.y)),
            mode,
            monitor: monitor
                .as_ref()
                .and_then(|handle| displays.monitor_id(handle)),
            display_mode,
            resize_policy: self.policy,
            resizable: self
                .capabilities
                .resize_policy_feedback
                .then(|| window.is_resizable()),
        }
    }
    pub(crate) fn submit(
        &mut self,
        window: &Window,
        displays: &NativeDisplays,
        id: u64,
        request: WindowRequest,
    ) -> WindowOperation {
        self.pending = None;
        let actual = self.state(window, displays);
        let prepared = match self.prepare(window, displays, request, actual) {
            Ok(prepared) => prepared,
            Err(error) => {
                return WindowOperation::Failed {
                    id,
                    error,
                    state: actual,
                };
            }
        };
        if actual.mode == WindowModeKind::Windowed
            && matches!(
                request.mode,
                Some(WindowMode::Borderless { .. } | WindowMode::Exclusive { .. })
            )
        {
            self.restore = Some((actual.size, actual.position));
        }
        if matches!(prepared.fullscreen, FullscreenChange::Set(Some(_))) {
            window.set_min_inner_size::<PhysicalSize<u32>>(None);
            window.set_max_inner_size::<PhysicalSize<u32>>(None);
        }
        if let FullscreenChange::Set(fullscreen) = prepared.fullscreen {
            super::native_result::set_fullscreen(window, fullscreen);
        }
        if let Some(policy) = prepared.policy {
            self.apply_resize_policy(window, policy);
        }
        if let Some(size) = prepared.expected.size.filter(|_| {
            !matches!(
                request.mode,
                Some(WindowMode::Borderless { .. } | WindowMode::Exclusive { .. })
            )
        }) {
            window.set_maximized(false);
            let _actual_size = window.request_inner_size(physical_size(size));
        }
        if let Some(position) = prepared.expected.position {
            window.set_outer_position(PhysicalPosition::new(position.0, position.1));
        }
        self.pending = Some(PendingWindow {
            id,
            expected: prepared.expected,
            started: Instant::now(),
        });
        WindowOperation::Pending { id }
    }
    fn apply_resize_policy(&mut self, window: &Window, policy: WindowResizePolicy) {
        let (min, max, resizable) = match policy {
            WindowResizePolicy::Fixed => (None, None, false),
            WindowResizePolicy::Resizable { min, max } => (min, max, true),
        };
        window.set_min_inner_size(min.map(physical_size));
        window.set_max_inner_size(max.map(physical_size));
        window.set_resizable(resizable);
        self.policy = policy;
    }
    fn prepare(
        &self,
        window: &Window,
        displays: &NativeDisplays,
        request: WindowRequest,
        actual: WindowState,
    ) -> Result<PreparedWindow, WindowOperationError> {
        request.validate()?;
        let mode = request.mode.map_or(actual.mode, |mode| match mode {
            WindowMode::Windowed => WindowModeKind::Windowed,
            WindowMode::Borderless { .. } => WindowModeKind::Borderless,
            WindowMode::Exclusive { .. } => WindowModeKind::Exclusive,
        });
        if mode != WindowModeKind::Windowed
            && (request.size.is_some()
                || request.placement.is_some()
                || request.resize_policy.is_some())
        {
            return Err(WindowOperationError::WindowedOnly);
        }
        if request.size.is_some() && !self.capabilities.size {
            return Err(WindowOperationError::Unsupported {
                capability: "physical client sizing",
            });
        }
        if request.resize_policy.is_some() && !self.capabilities.resize_policy {
            return Err(WindowOperationError::Unsupported {
                capability: "resize policy",
            });
        }
        if request.placement.is_some() && !self.capabilities.placement {
            return Err(WindowOperationError::Unsupported {
                capability: "desktop placement",
            });
        }
        let policy = request.resize_policy.unwrap_or(self.policy);
        let restoring = actual.mode != WindowModeKind::Windowed && mode == WindowModeKind::Windowed;
        let size = request.size.or_else(|| {
            restoring
                .then(|| self.restore.map(|value| value.0))
                .flatten()
        });
        if let WindowResizePolicy::Resizable { min, max } = policy {
            validate_constraints(min, max, size)?;
        }
        let mut expected = ExpectedWindow {
            mode: request.mode.map(|_| mode),
            size,
            resizable: request
                .resize_policy
                .or_else(|| restoring.then_some(self.policy))
                .and_then(|policy| {
                    self.capabilities
                        .resize_policy_feedback
                        .then_some(!matches!(policy, WindowResizePolicy::Fixed))
                }),
            ..ExpectedWindow::default()
        };
        let fullscreen = self.prepare_fullscreen(displays, request.mode, &mut expected)?;
        expected.position = match request.placement {
            Some(WindowPlacement::Position { x, y }) => Some((x, y)),
            Some(WindowPlacement::Centered { monitor }) => {
                let handle = displays
                    .monitor_handle(monitor)
                    .ok_or(WindowOperationError::MonitorUnavailable { monitor })?;
                let origin = handle.position();
                let extent = handle.size();
                let outer = window.outer_size();
                let client = window.inner_size();
                let desired = size.unwrap_or(actual.size);
                let outer = (
                    desired
                        .width
                        .saturating_add(outer.width.saturating_sub(client.width)),
                    desired
                        .height
                        .saturating_add(outer.height.saturating_sub(client.height)),
                );
                expected.monitor = Some(monitor);
                Some(centered_origin(
                    (origin.x, origin.y),
                    (extent.width, extent.height),
                    outer,
                ))
            }
            None if restoring && self.capabilities.placement => {
                self.restore.and_then(|value| value.1)
            }
            None => None,
        };
        Ok(PreparedWindow {
            fullscreen,
            policy: request
                .resize_policy
                .or_else(|| restoring.then_some(self.policy)),
            expected,
        })
    }
    fn prepare_fullscreen(
        &self,
        displays: &NativeDisplays,
        mode: Option<WindowMode>,
        expected: &mut ExpectedWindow,
    ) -> Result<FullscreenChange, WindowOperationError> {
        let fullscreen = match mode {
            None => FullscreenChange::Preserve,
            Some(WindowMode::Windowed) => FullscreenChange::Set(None),
            Some(WindowMode::Borderless { monitor }) => {
                if !self.capabilities.borderless {
                    return Err(WindowOperationError::Unsupported {
                        capability: "borderless fullscreen",
                    });
                }
                let handle = displays
                    .monitor_handle(monitor)
                    .ok_or(WindowOperationError::MonitorUnavailable { monitor })?;
                let extent = handle.size();
                expected.monitor = Some(monitor);
                expected.size = Some(DisplayResolution {
                    width: extent.width,
                    height: extent.height,
                });
                FullscreenChange::Set(Some(Fullscreen::Borderless(Some(handle))))
            }
            Some(WindowMode::Exclusive { monitor, mode }) => {
                if !self.capabilities.exclusive {
                    return Err(WindowOperationError::Unsupported {
                        capability: "exclusive fullscreen",
                    });
                }
                let handle = displays
                    .monitor_handle(monitor)
                    .ok_or(WindowOperationError::MonitorUnavailable { monitor })?;
                let native_mode = handle
                    .video_modes()
                    .find(|candidate| {
                        candidate.size().width == mode.resolution.width
                            && candidate.size().height == mode.resolution.height
                            && candidate.refresh_rate_millihertz() == mode.refresh_rate_millihertz
                            && candidate.bit_depth() == mode.bit_depth
                    })
                    .ok_or(WindowOperationError::ModeUnavailable { monitor, mode })?;
                expected.monitor = Some(monitor);
                expected.display_mode = Some(mode);
                expected.size = Some(mode.resolution);
                FullscreenChange::Set(Some(Fullscreen::Exclusive(native_mode)))
            }
        };
        Ok(fullscreen)
    }
    pub(crate) fn feedback(
        &mut self,
        window: &Window,
        displays: &NativeDisplays,
    ) -> Option<WindowOperation> {
        let pending = self.pending.as_ref()?;
        let result = pending.observe(self.state(window, displays), Instant::now());
        if result.is_some() {
            self.pending = None;
        }
        result
    }
    pub(crate) fn cancel(
        &mut self,
        window: &Window,
        displays: &NativeDisplays,
    ) -> Option<WindowOperation> {
        let pending = self.pending.take()?;
        Some(WindowOperation::Failed {
            id: pending.id,
            error: WindowOperationError::Superseded,
            state: self.state(window, displays),
        })
    }
}
fn physical_size(size: DisplayResolution) -> PhysicalSize<u32> {
    PhysicalSize::new(size.width, size.height)
}
