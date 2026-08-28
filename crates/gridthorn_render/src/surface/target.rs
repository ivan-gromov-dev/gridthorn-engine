/// Owned platform handle used to create a GPU presentation surface.
pub struct WindowSurfaceTarget(wgpu::SurfaceTarget<'static>);

impl WindowSurfaceTarget {
    /// Wrap an owned native window handle for renderer initialization.
    pub fn new<T>(target: T) -> Self
    where
        T: wgpu::DisplayAndWindowHandle + 'static,
    {
        Self(wgpu::SurfaceTarget::DisplayAndWindow(Box::new(target)))
    }

    pub(super) fn into_wgpu(self) -> wgpu::SurfaceTarget<'static> {
        self.0
    }
}
