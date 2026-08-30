use wgpu::util::{DeviceExt, TextureDataOrder};
use wgpu::{
    BindGroup, BindGroupLayout, BlendState, Color, ColorTargetState, ColorWrites, CommandEncoder,
    Device, FragmentState, LoadOp, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, StoreOp, TextureFormat, TextureView, VertexBufferLayout, VertexState,
    VertexStepMode,
};

use crate::RenderFrame;
use crate::presentation::{FrameGeometry, SpriteVertex, textured_sprite_batches};

use super::lifecycle::SurfaceExtent;

pub(super) struct SpritePipeline {
    pipeline: RenderPipeline,
    textured_pipeline: RenderPipeline,
    texture_layout: BindGroupLayout,
}

impl SpritePipeline {
    pub(super) fn new(device: &Device, format: TextureFormat) -> Self {
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
    ) {
        let geometry = FrameGeometry::new(frame, extent.width, extent.height);
        let vertex_buffer = (!geometry.vertices.is_empty()).then(|| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("gridthorn sprite vertices"),
                contents: bytemuck::cast_slice(&geometry.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        });
        let textured = textured_sprite_batches(frame, extent.width, extent.height)
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
                    layout: &self.texture_layout,
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
            .collect::<Vec<(wgpu::Buffer, BindGroup, u32)>>();
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
            let vertex_count = u32::try_from(geometry.vertices.len()).unwrap_or(u32::MAX);
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..vertex_count, 0..1);
        }
        for (buffer, bind_group, vertex_count) in &textured {
            render_pass.set_pipeline(&self.textured_pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.set_vertex_buffer(0, buffer.slice(..));
            render_pass.draw(0..*vertex_count, 0..1);
        }
    }
}
