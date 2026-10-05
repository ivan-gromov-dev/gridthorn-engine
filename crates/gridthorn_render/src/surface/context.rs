use tracing::{debug, info, warn};
use wgpu::{
    Adapter, CommandEncoderDescriptor, Device, DeviceDescriptor, Instance, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureViewDescriptor,
};

use crate::RenderFrame;

use super::errors::RenderSurfaceError;
use super::lifecycle::{SurfaceChange, SurfaceExtent, SurfaceLifecycle};
use super::pipeline::SpritePipeline;
use super::target::WindowSurfaceTarget;

/// GPU renderer bound to one owned window surface.
pub struct SurfaceRenderer {
    adapter: Adapter,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    configuration: Option<SurfaceConfiguration>,
    lifecycle: SurfaceLifecycle,
    frame: RenderFrame,
    sprite_pipeline: Option<SpritePipeline>,
    performance: super::performance::SurfacePerformance,
}

impl SurfaceRenderer {
    /// Create the GPU surface, adapter, and device for a platform target.
    ///
    /// # Errors
    ///
    /// Returns an error when surface creation, adapter selection, device
    /// creation, or initial surface compatibility validation fails.
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
        let performance = super::performance::SurfacePerformance::new();
        let required_features = if performance.enabled() {
            adapter.features() & wgpu::Features::TIMESTAMP_QUERY
        } else {
            wgpu::Features::empty()
        };
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            required_features,
            ..DeviceDescriptor::default()
        }))
        .map_err(RenderSurfaceError::device_request)?;
        let adapter_info = adapter.get_info();
        info!(
            component = "renderer",
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
            frame: RenderFrame::default(),
            sprite_pipeline: None,
            performance,
        };
        renderer.resize(width, height)?;
        Ok(renderer)
    }

    /// Queue a new non-zero extent or suspend acquisition at zero.
    /// Native configuration is deferred until the next renderable frame.
    ///
    /// # Errors
    ///
    /// Returns an error when the selected adapter cannot configure the new
    /// non-zero surface extent.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), RenderSurfaceError> {
        match self.lifecycle.resize(width, height) {
            SurfaceChange::QueueConfiguration(extent) => {
                self.surface
                    .get_default_config(&self.adapter, extent.width, extent.height)
                    .ok_or(RenderSurfaceError::UnsupportedConfiguration)?;
            }
            SurfaceChange::Suspend => {
                self.configuration = None;
                debug!(component = "renderer", "suspended zero-sized surface");
            }
            SurfaceChange::Unchanged => {}
        }
        Ok(())
    }

    /// Pause or resume frame acquisition for platform occlusion.
    pub fn set_occluded(&mut self, occluded: bool) {
        self.lifecycle.set_occluded(occluded);
        debug!(
            component = "renderer",
            occluded, "surface occlusion changed"
        );
    }

    /// Replace the immutable presentation snapshot used by the next frame.
    pub fn set_frame(&mut self, frame: RenderFrame) {
        self.frame = frame;
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

        if let Some(extent) = self.lifecycle.configuration_required() {
            self.configure(extent)?;
            self.lifecycle.mark_configured();
        }

        let acquire_start = self.performance.clock();
        let (frame, suboptimal) = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => (frame, false),
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => (frame, true),
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.lifecycle.invalidate_configuration();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => return Err(RenderSurfaceError::SurfaceLost),
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(RenderSurfaceError::SurfaceValidation);
            }
        };
        let acquire_time = acquire_start.map(|start| start.elapsed());
        let encode_start = self.performance.clock();
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("gridthorn surface encoder"),
            });
        let extent = self
            .lifecycle
            .extent()
            .ok_or(RenderSurfaceError::UnsupportedConfiguration)?;
        let pipeline = self
            .sprite_pipeline
            .as_mut()
            .ok_or(RenderSurfaceError::UnsupportedConfiguration)?;
        let pipeline_sample = pipeline.encode(
            &self.device,
            &self.queue,
            &mut encoder,
            &view,
            &self.frame,
            extent,
        );
        let encode_time = encode_start.map(|start| start.elapsed());
        let submit_start = self.performance.clock();
        self.queue.submit([encoder.finish()]);
        pipeline.submitted();
        let submit_time = submit_start.map(|start| start.elapsed());
        let present_start = self.performance.clock();
        self.queue.present(frame);
        if let Some(acquire) = acquire_time {
            self.performance.record(super::performance::FrameSample {
                acquire,
                pipeline: pipeline_sample.unwrap_or_default(),
                encode: encode_time.unwrap_or_default(),
                submit: submit_time.unwrap_or_default(),
                present: present_start
                    .map(|start| start.elapsed())
                    .unwrap_or_default(),
            });
        }

        if suboptimal {
            warn!(
                component = "renderer",
                "surface frame was suboptimal; reconfiguring"
            );
            self.lifecycle.invalidate_configuration();
        }
        Ok(())
    }

    fn configure(&mut self, extent: SurfaceExtent) -> Result<(), RenderSurfaceError> {
        let configuration = self
            .surface
            .get_default_config(&self.adapter, extent.width, extent.height)
            .ok_or(RenderSurfaceError::UnsupportedConfiguration)?;
        self.surface.configure(&self.device, &configuration);
        if self.performance.enabled() {
            let adapter = self.adapter.get_info();
            eprintln!(
                "render_configuration,adapter={:?},backend={:?},driver={:?},driver_info={:?},width={},height={},present={:?},timestamp_query={}",
                adapter.name,
                adapter.backend,
                adapter.driver,
                adapter.driver_info,
                extent.width,
                extent.height,
                configuration.present_mode,
                self.device
                    .features()
                    .contains(wgpu::Features::TIMESTAMP_QUERY),
            );
        }
        self.sprite_pipeline = Some(SpritePipeline::new(
            &self.device,
            configuration.format,
            self.performance.enabled(),
            &self.queue,
            self.performance.next_frame(),
        ));
        self.configuration = Some(configuration);
        debug!(
            component = "renderer",
            width = extent.width,
            height = extent.height,
            "configured surface"
        );
        Ok(())
    }
}
