use super::lifecycle::SurfaceExtent;
use crate::{
    Camera2d, RenderFrame, Sprite, TimingOverlay, UiPrimitive, presentation::FrameGeometry,
};
use wgpu::util::DeviceExt;

/// Retain one colored/UI geometry snapshot and its immutable GPU vertex buffer.
pub(super) struct ColoredFrame {
    previous: Option<(ColoredInputs, SurfaceExtent)>,
    geometry: FrameGeometry,
    buffer: Option<wgpu::Buffer>,
}

impl ColoredFrame {
    pub fn new() -> Self {
        Self {
            previous: None,
            geometry: FrameGeometry::new(&RenderFrame::default(), 0, 0),
            buffer: None,
        }
    }

    pub fn prepare(&mut self, frame: &RenderFrame, extent: SurfaceExtent) -> bool {
        if self
            .previous
            .as_ref()
            .is_some_and(|(previous, size)| *size == extent && same_colored_frame(previous, frame))
        {
            return false;
        }
        self.geometry.update(frame, extent.width, extent.height);
        self.previous = Some((
            ColoredInputs {
                camera: frame.camera(),
                sprites: frame.sprites().to_vec(),
                overlay: frame.timing_overlay(),
                ui: frame.ui().to_vec(),
            },
            extent,
        ));
        true
    }

    pub fn upload(&mut self, device: &wgpu::Device, changed: bool) {
        if changed {
            self.buffer = (!self.geometry.vertices.is_empty()).then(|| {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("gridthorn sprite vertices"),
                    contents: bytemuck::cast_slice(&self.geometry.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            });
        }
    }

    pub fn geometry(&self) -> &FrameGeometry {
        &self.geometry
    }

    pub fn buffer(&self) -> Option<&wgpu::Buffer> {
        self.buffer.as_ref()
    }
}

struct ColoredInputs {
    camera: Camera2d,
    sprites: Vec<Sprite>,
    overlay: Option<TimingOverlay>,
    ui: Vec<UiPrimitive>,
}

fn same_colored_frame(left: &ColoredInputs, right: &RenderFrame) -> bool {
    left.camera == right.camera()
        && left.sprites == right.sprites()
        && left.overlay == right.timing_overlay()
        && same_ui(&left.ui, right.ui())
}

fn same_ui(left: &[UiPrimitive], right: &[UiPrimitive]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| match (left, right) {
                (UiPrimitive::ShapedText(left), UiPrimitive::ShapedText(right)) => {
                    left.same_draw_data(right)
                }
                (
                    UiPrimitive::Clipped {
                        bounds: left,
                        children: left_children,
                    },
                    UiPrimitive::Clipped {
                        bounds: right,
                        children: right_children,
                    },
                ) => left == right && same_ui(left_children, right_children),
                _ => left == right,
            })
}

#[cfg(test)]
#[path = "colored_frame/test/mod.rs"]
mod test;
