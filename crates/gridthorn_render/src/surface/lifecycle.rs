/// A non-zero renderable surface extent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SurfaceExtent {
    pub(super) width: u32,
    pub(super) height: u32,
}

/// Work required after a window size transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SurfaceChange {
    Configure(SurfaceExtent),
    Suspend,
    Unchanged,
}

/// Platform-independent surface lifecycle state.
#[derive(Debug, Default)]
pub(super) struct SurfaceLifecycle {
    extent: Option<SurfaceExtent>,
    occluded: bool,
}

impl SurfaceLifecycle {
    /// Apply a new physical window size.
    pub(super) fn resize(&mut self, width: u32, height: u32) -> SurfaceChange {
        let next_extent = non_zero_extent(width, height);
        if next_extent == self.extent {
            return SurfaceChange::Unchanged;
        }

        self.extent = next_extent;
        next_extent.map_or(SurfaceChange::Suspend, SurfaceChange::Configure)
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
