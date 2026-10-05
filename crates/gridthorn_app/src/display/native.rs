use super::{
    DisplayMode, DisplayResolution, Displays, MonitorId, MonitorInfo, MonitorSelection,
    MonitorSelectionError,
    catalog::DisplayCatalog,
    selection::{PendingSelection, centered_origin},
};
use std::time::Instant;
use winit::{event_loop::ActiveEventLoop, monitor::MonitorHandle, window::Window};

#[derive(Default)]
pub(crate) struct NativeDisplays {
    catalog: Option<DisplayCatalog<MonitorHandle>>,
    pending: Option<PendingSelection>,
}

impl NativeDisplays {
    pub(crate) fn refresh(&mut self, event_loop: &ActiveEventLoop, window: &Window) -> Displays {
        let observations = event_loop
            .available_monitors()
            .map(|handle| {
                let size = handle.size();
                let position = handle.position();
                let info = MonitorInfo {
                    id: MonitorId(0, 0),
                    name: handle.name(),
                    resolution: DisplayResolution {
                        width: size.width,
                        height: size.height,
                    },
                    position: (position.x, position.y),
                    refresh_rate_millihertz: handle.refresh_rate_millihertz(),
                    scale_factor: handle.scale_factor(),
                    modes: handle
                        .video_modes()
                        .map(|mode| {
                            let size = mode.size();
                            DisplayMode {
                                resolution: DisplayResolution {
                                    width: size.width,
                                    height: size.height,
                                },
                                refresh_rate_millihertz: mode.refresh_rate_millihertz(),
                                bit_depth: mode.bit_depth(),
                            }
                        })
                        .collect(),
                };
                (handle, info)
            })
            .collect();
        let catalog = self.catalog.get_or_insert_with(DisplayCatalog::default);
        let mut inventory = catalog.sample(observations, event_loop.primary_monitor().as_ref());
        inventory.active = window
            .current_monitor()
            .as_ref()
            .and_then(|key| catalog.id(key));
        inventory
    }

    pub(crate) fn select(&mut self, window: &Window, monitor: MonitorId) -> MonitorSelection {
        self.pending = None;
        match self.place(window, monitor) {
            Ok(()) => {
                self.pending = Some(PendingSelection::new(monitor, Instant::now()));
                MonitorSelection::Pending { monitor }
            }
            Err(error) => MonitorSelection::Failed { monitor, error },
        }
    }

    fn place(&self, window: &Window, monitor: MonitorId) -> Result<(), MonitorSelectionError> {
        let handle = self
            .catalog
            .as_ref()
            .and_then(|catalog| catalog.key(monitor))
            .ok_or(MonitorSelectionError::Unavailable { monitor })?;
        if window.fullscreen().is_some() {
            return Err(MonitorSelectionError::FullscreenUnsupported);
        }
        window
            .outer_position()
            .map_err(|error| MonitorSelectionError::PositionUnavailable {
                reason: error.to_string(),
            })?;
        let origin = handle.position();
        let extent = handle.size();
        let size = window.outer_size();
        let (x, y) = centered_origin(
            (origin.x, origin.y),
            (extent.width, extent.height),
            (size.width, size.height),
        );
        window.set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
        Ok(())
    }

    pub(crate) fn selection_feedback(&mut self, window: &Window) -> Option<MonitorSelection> {
        let pending = self.pending.as_ref()?;
        let actual = window
            .current_monitor()
            .as_ref()
            .and_then(|key| self.catalog.as_ref()?.id(key));
        let result = pending.observe(actual, Instant::now());
        if result.is_some() {
            self.pending = None;
        }
        result
    }
}
