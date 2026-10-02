/// A non-zero renderable surface extent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SurfaceExtent {
    pub(super) width: u32,
    pub(super) height: u32,
}

/// Work required after a window size transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SurfaceChange {
    QueueConfiguration(SurfaceExtent),
    Suspend,
    Unchanged,
}

/// Platform-independent surface lifecycle state.
#[derive(Debug, Default)]
pub(super) struct SurfaceLifecycle {
    extent: Option<SurfaceExtent>,
    occluded: bool,
    configuration_pending: bool,
}

impl SurfaceLifecycle {
    /// Apply a new physical window size.
    pub(super) fn resize(&mut self, width: u32, height: u32) -> SurfaceChange {
        let next_extent = non_zero_extent(width, height);
        if next_extent == self.extent {
            return SurfaceChange::Unchanged;
        }

        self.extent = next_extent;
        self.configuration_pending = next_extent.is_some();
        next_extent.map_or(SurfaceChange::Suspend, SurfaceChange::QueueConfiguration)
    }

    /// Return the latest extent requiring configuration immediately before acquisition.
    /// Deferral avoids creating unpresented swapchains during startup or resize-only shutdown.
    pub(super) fn configuration_required(&self) -> Option<SurfaceExtent> {
        if self.configuration_pending && self.can_render() {
            self.extent
        } else {
            None
        }
    }

    /// Record a successful native configuration.
    pub(super) fn mark_configured(&mut self) {
        self.configuration_pending = false;
    }

    /// Queue recovery without creating an unpresented replacement swapchain.
    pub(super) fn invalidate_configuration(&mut self) {
        self.configuration_pending = self.extent.is_some();
    }

    /// Record whether the platform currently occludes the window.
    pub(super) fn set_occluded(&mut self, occluded: bool) {
        self.occluded = occluded;
    }

    /// Return whether a frame can be acquired safely.
    pub(super) fn can_render(&self) -> bool {
        self.extent.is_some() && !self.occluded
    }

    /// Return the current non-zero extent.
    pub(super) fn extent(&self) -> Option<SurfaceExtent> {
        self.extent
    }
}

fn non_zero_extent(width: u32, height: u32) -> Option<SurfaceExtent> {
    (width > 0 && height > 0).then_some(SurfaceExtent { width, height })
}

#[cfg(test)]
mod test;
