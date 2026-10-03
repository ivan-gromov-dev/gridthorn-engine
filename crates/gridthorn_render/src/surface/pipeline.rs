use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupLayout, BlendState, Color, ColorTargetState, ColorWrites, CommandEncoder, Device,
    FragmentState, LoadOp, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, StoreOp, TextureFormat, TextureView, VertexBufferLayout, VertexState,
    VertexStepMode,
};

use crate::RenderFrame;
use crate::presentation::{FrameGeometry, SpriteVertex};

use super::lifecycle::SurfaceExtent;

pub(super) struct SpritePipeline {
    pipeline: RenderPipeline,
    textured_pipeline: RenderPipeline,
    texture_layout: BindGroupLayout,
    performance: bool,
}

impl SpritePipeline {
    pub(super) fn new(device: &Device, format: TextureFormat, performance: bool) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("gridthorn sprite shader"),
            source: ShaderSource::Wgsl(include_str!("sprite.wgsl").into()),
        });
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gridthorn texture layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("gridthorn sprite pipeline layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });
        let sampled_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("gridthorn textured sprite pipeline layout"),
            bind_group_layouts: &[Some(&texture_layout)],
            immediate_size: 0,
        });
        let attributes = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32x2];
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("gridthorn sprite pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Some(VertexBufferLayout {
                    array_stride: size_of::<SpriteVertex>() as wgpu::BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &attributes,
                })],
            },
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let textured_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("gridthorn textured sprite pipeline"),
            layout: Some(&sampled_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Some(VertexBufferLayout {
                    array_stride: size_of::<SpriteVertex>() as wgpu::BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &attributes,
                })],
            },
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_textured"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            textured_pipeline,
            texture_layout,
            performance,
        }
    }

    pub(super) fn encode(
        &self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        frame: &RenderFrame,
        extent: SurfaceExtent,
    ) -> Option<super::performance::PipelineSample> {
        let geometry_start = self.performance.then(std::time::Instant::now);
        let geometry = FrameGeometry::new(frame, extent.width, extent.height);
        let geometry_time = geometry_start.map(|start| start.elapsed());
        let resources_start = self.performance.then(std::time::Instant::now);
        let vertex_buffer = (!geometry.vertices.is_empty()).then(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("gridthorn sprite vertices"),
                contents: bytemuck::cast_slice(&geometry.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        });
        let textured =
            super::uploads::textured_resources(device, queue, &self.texture_layout, frame, extent);
        let resources_time = resources_start.map(|start| start.elapsed());
        let color_attachment = RenderPassColorAttachment {
            view,
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
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("gridthorn sprite pass"),
            color_attachments: &[Some(color_attachment)],
            ..RenderPassDescriptor::default()
        });
        if let Some(vertex_buffer) = vertex_buffer.as_ref() {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..geometry.world_vertex_count, 0..1);
        }
        for (buffer, bind_group, vertex_count) in &textured {
            render_pass.set_pipeline(&self.textured_pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.set_vertex_buffer(0, buffer.slice(..));
            render_pass.draw(0..*vertex_count, 0..1);
        }
        if let Some(vertex_buffer) = vertex_buffer.as_ref() {
            let vertex_count = u32::try_from(geometry.vertices.len()).unwrap_or(u32::MAX);
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(geometry.world_vertex_count..vertex_count, 0..1);
        }
        geometry_time.map(|geometry_time| {
            let vertices = geometry.vertices.len()
                + textured
                    .iter()
                    .map(|(_, _, count)| *count as usize)
                    .sum::<usize>();
            super::performance::PipelineSample {
                geometry: geometry_time,
                resources: resources_time.unwrap_or_default(),
                vertices,
                vertex_bytes: vertices * std::mem::size_of::<SpriteVertex>(),
            }
        })
    }
}
