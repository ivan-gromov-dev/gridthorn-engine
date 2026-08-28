use tracing::{debug, info, warn};
use wgpu::{
    Adapter, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Instance, LoadOp,
    Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions,
    StoreOp, Surface, SurfaceConfiguration, TextureViewDescriptor,
};

use super::error::RenderSurfaceError;
use super::lifecycle::{SurfaceChange, SurfaceExtent, SurfaceLifecycle};
use super::target::WindowSurfaceTarget;

/// GPU renderer bound to one owned window surface.
pub struct SurfaceRenderer {
    adapter: Adapter,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    configuration: Option<SurfaceConfiguration>,
    lifecycle: SurfaceLifecycle,
}

impl SurfaceRenderer {
    /// Create the GPU surface, adapter, and device for a platform target.
    ///
    /// # Errors
    ///
    /// Returns an error when surface creation, adapter selection, device
    /// creation, or initial surface configuration fails.
    pub fn new(
        target: WindowSurfaceTarget,
        width: u32,
        height: u32,
    ) -> Result<Self, RenderSurfaceError> {
        let instance = Instance::default();
        let surface = instance
            .create_surface(target.into_wgpu())
            .map_err(RenderSurfaceError::surface_creation)?;
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..RequestAdapterOptions::default()
        }))
        .map_err(RenderSurfaceError::adapter_request)?;
        let (device, queue) =
            pollster::block_on(adapter.request_device(&DeviceDescriptor::default()))
                .map_err(RenderSurfaceError::device_request)?;
        let adapter_info = adapter.get_info();
        info!(
            adapter = %adapter_info.name,
            backend = ?adapter_info.backend,
            "initialized graphics adapter"
        );

        let mut renderer = Self {
            adapter,
            device,
            queue,
            surface,
            configuration: None,
            lifecycle: SurfaceLifecycle::default(),
        };
        renderer.resize(width, height)?;
        Ok(renderer)
    }

    /// Reconfigure for a new non-zero extent or suspend acquisition at zero.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected adapter cannot configure the new
    /// non-zero surface extent.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), RenderSurfaceError> {
        match self.lifecycle.resize(width, height) {
            SurfaceChange::Configure(extent) => self.configure(extent)?,
            SurfaceChange::Suspend => {
                self.configuration = None;
                debug!("suspended zero-sized surface");
            }
            SurfaceChange::Unchanged => {}
        }
        Ok(())
    }

    /// Pause or resume frame acquisition for platform occlusion.
    pub fn set_occluded(&mut self, occluded: bool) {
        self.lifecycle.set_occluded(occluded);
        debug!(occluded, "surface occlusion changed");
    }

    /// Clear and present one frame when the surface is renderable.
    ///
    /// # Errors
    ///
    /// Returns an error when frame acquisition fails permanently or the
    /// surface cannot be reconfigured after becoming outdated or lost.
    pub fn render(&mut self) -> Result<(), RenderSurfaceError> {
        if !self.lifecycle.can_render() {
            return Ok(());
        }

        let (frame, suboptimal) = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => (frame, false),
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => (frame, true),
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.reconfigure_current()?;
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => return Err(RenderSurfaceError::SurfaceLost),
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(RenderSurfaceError::SurfaceValidation);
            }
        };
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("gridthorn surface encoder"),
            });
        {
            let color_attachment = RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.04,
                        g: 0.10,
                        b: 0.16,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            };
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("gridthorn surface clear pass"),
                color_attachments: &[Some(color_attachment)],
                ..RenderPassDescriptor::default()
            });
        }
        self.queue.submit([encoder.finish()]);
        self.queue.present(frame);

        if suboptimal {
            warn!("surface frame was suboptimal; reconfiguring");
            self.reconfigure_current()?;
        }
        Ok(())
    }

    fn configure(&mut self, extent: SurfaceExtent) -> Result<(), RenderSurfaceError> {
        let configuration = self
            .surface
            .get_default_config(&self.adapter, extent.width, extent.height)
            .ok_or(RenderSurfaceError::UnsupportedConfiguration)?;
        self.surface.configure(&self.device, &configuration);
        self.configuration = Some(configuration);
        debug!(
            width = extent.width,
            height = extent.height,
            "configured surface"
        );
        Ok(())
    }

    fn reconfigure_current(&mut self) -> Result<(), RenderSurfaceError> {
        if let Some(extent) = self.lifecycle.extent() {
            self.configure(extent)?;
        }
        Ok(())
    }
}
