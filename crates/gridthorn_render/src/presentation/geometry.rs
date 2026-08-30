use bytemuck::{Pod, Zeroable};

use super::RenderFrame;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct SpriteVertex {
    pub(crate) position: [f32; 2],
    pub(crate) color: [f32; 4],
}

pub(crate) struct FrameGeometry {
    pub(crate) vertices: Vec<SpriteVertex>,
}

impl FrameGeometry {
    #[expect(
        clippy::cast_precision_loss,
        reason = "surface dimensions become f32 GPU clip-space coordinates"
    )]
    pub(crate) fn new(frame: &RenderFrame, width: u32, height: u32) -> Self {
        let camera = frame.camera();
        let viewport_height = camera.viewport_height();
        if width == 0 || height == 0 || !viewport_height.is_finite() || viewport_height <= 0.0 {
            return Self {
                vertices: Vec::new(),
            };
        }
        let aspect = width as f32 / height as f32;
        let half_height = viewport_height * 0.5;
        let half_width = half_height * aspect;
        let center = camera.center();
        let mut vertices = Vec::with_capacity(frame.sprites().len() * 6);
        for sprite in frame.sprites() {
            let position = sprite.position();
            let size = sprite.size();
            if !position.into_iter().all(f32::is_finite)
                || !size
                    .into_iter()
                    .all(|value| value.is_finite() && value > 0.0)
            {
                continue;
            }
            let left = (position[0] - size[0] * 0.5 - center[0]) / half_width;
            let right = (position[0] + size[0] * 0.5 - center[0]) / half_width;
            let top = -(position[1] - size[1] * 0.5 - center[1]) / half_height;
            let bottom = -(position[1] + size[1] * 0.5 - center[1]) / half_height;
            let color = sprite.color().components();
            vertices.extend([
                vertex(left, top, color),
                vertex(left, bottom, color),
                vertex(right, bottom, color),
                vertex(left, top, color),
                vertex(right, bottom, color),
                vertex(right, top, color),
            ]);
        }
        Self { vertices }
    }
}

fn vertex(x: f32, y: f32, color: [f32; 4]) -> SpriteVertex {
    SpriteVertex {
        position: [x, y],
        color,
    }
}

#[cfg(test)]
mod test;
