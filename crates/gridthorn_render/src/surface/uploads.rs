use super::lifecycle::SurfaceExtent;
use crate::{RenderFrame, presentation::textured_sprite_batches};
use wgpu::util::{DeviceExt, TextureDataOrder};
use wgpu::{BindGroup, BindGroupLayout, Device, Queue};

pub(super) fn textured_resources(
    device: &Device,
    queue: &Queue,
    texture_layout: &BindGroupLayout,
    frame: &RenderFrame,
    extent: SurfaceExtent,
) -> Vec<(wgpu::Buffer, BindGroup, u32)> {
    textured_sprite_batches(frame, extent.width, extent.height)
        .into_iter()
        .map(|batch| {
            let dimensions = batch.texture.dimensions();
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
                batch.texture.rgba8(),
            );
            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                ..wgpu::SamplerDescriptor::default()
            });
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
            });
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("gridthorn textured sprite vertices"),
                contents: bytemuck::cast_slice(&batch.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let vertex_count = u32::try_from(batch.vertices.len()).unwrap_or(u32::MAX);
            (buffer, bind_group, vertex_count)
        })
        .collect::<Vec<(wgpu::Buffer, BindGroup, u32)>>()
}
