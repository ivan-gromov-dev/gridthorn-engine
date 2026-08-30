use wgpu::util::DeviceExt;
use wgpu::{
    BlendState, Color, ColorTargetState, ColorWrites, CommandEncoder, Device, FragmentState,
    LoadOp, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StoreOp, TextureFormat,
    TextureView, VertexBufferLayout, VertexState, VertexStepMode,
};

use crate::RenderFrame;
use crate::presentation::{FrameGeometry, SpriteVertex};

use super::lifecycle::SurfaceExtent;

pub(super) struct SpritePipeline {
    pipeline: RenderPipeline,
}

impl SpritePipeline {
    pub(super) fn new(device: &Device, format: TextureFormat) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("gridthorn sprite shader"),
            source: ShaderSource::Wgsl(include_str!("sprite.wgsl").into()),
        });
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("gridthorn sprite pipeline layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });
        let attributes = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];
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
        Self { pipeline }
    }

    pub(super) fn encode(
        &self,
        device: &Device,
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
    }
}
