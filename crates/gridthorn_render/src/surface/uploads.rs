use super::lifecycle::SurfaceExtent;
use super::texture_cache::TextureCache;
use crate::{RenderFrame, presentation::textured_sprite_batches};
use gridthorn_assets::TextureAsset;
use wgpu::util::{DeviceExt, TextureDataOrder};
use wgpu::{BindGroup, BindGroupLayout, Device, Queue};

/// Retain visible decoded textures while preserving adjacent batch painter order.
pub(super) struct TexturedResources {
    textures: TextureCache<BindGroup>,
    frame: super::textured_frame::TexturedFrame,
    pub batches: Vec<(wgpu::Buffer, BindGroup, u32)>,
    pub uploaded_texture_bytes: usize,
    pub changed: bool,
}

impl TexturedResources {
    pub fn new() -> Self {
        Self {
            textures: TextureCache::new(),
            frame: super::textured_frame::TexturedFrame::new(),
            batches: Vec::new(),
            uploaded_texture_bytes: 0,
            changed: false,
        }
    }

    pub fn prepare(
        &mut self,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        frame: &RenderFrame,
        extent: SurfaceExtent,
    ) {
        self.uploaded_texture_bytes = 0;
        self.changed = self.frame.prepare(frame, extent);
        if !self.changed {
            return;
        }
        let batches = textured_sprite_batches(frame, extent.width, extent.height);
        self.textures.retain(
            &batches
                .iter()
                .map(|batch| batch.texture)
                .collect::<Vec<_>>(),
        );
        self.batches.clear();
        for batch in batches {
            let (binding, inserted) = self.textures.get_or_insert(batch.texture, || {
                texture_binding(device, queue, layout, batch.texture)
            });
            if inserted {
                self.uploaded_texture_bytes += batch.texture.rgba8().len();
            }
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("gridthorn textured sprite vertices"),
                contents: bytemuck::cast_slice(&batch.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.batches.push((
                buffer,
                binding.clone(),
                u32::try_from(batch.vertices.len()).unwrap_or(u32::MAX),
            ));
        }
    }

    pub fn texture_count(&self) -> usize {
        self.textures.len()
    }
    pub fn texture_bytes(&self) -> usize {
        self.textures.bytes()
    }
}

fn texture_binding(
    device: &Device,
    queue: &Queue,
    texture_layout: &BindGroupLayout,
    asset: &TextureAsset,
) -> BindGroup {
    let dimensions = asset.dimensions();
    let texture = device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: Some("gridthorn sprite texture"),
            size: wgpu::Extent3d {
                width: dimensions[0],
                height: dimensions[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        TextureDataOrder::LayerMajor,
        asset.rgba8(),
    );
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        ..wgpu::SamplerDescriptor::default()
    });
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("gridthorn sprite texture bind group"),
        layout: texture_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    })
}

#[cfg(test)]
#[path = "uploads/test/mod.rs"]
mod test;
