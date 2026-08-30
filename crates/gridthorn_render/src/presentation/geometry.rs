use bytemuck::{Pod, Zeroable};

use gridthorn_assets::TextureAsset;

use super::RenderFrame;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct SpriteVertex {
    pub(crate) position: [f32; 2],
    pub(crate) color: [f32; 4],
    pub(crate) uv: [f32; 2],
}

pub(crate) struct FrameGeometry {
    pub(crate) vertices: Vec<SpriteVertex>,
}

pub(crate) struct TexturedSpriteBatch<'texture> {
    pub(crate) texture: &'texture TextureAsset,
    pub(crate) vertices: Vec<SpriteVertex>,
}

pub(crate) fn textured_sprite_batches(
    frame: &RenderFrame,
    width: u32,
    height: u32,
) -> Vec<TexturedSpriteBatch<'_>> {
    let mut batches: Vec<TexturedSpriteBatch<'_>> = Vec::new();
    for sprite in frame.textured_sprites() {
        let vertices = projected_vertices(
            frame.camera(),
            sprite.position(),
            sprite.size(),
            sprite.tint().components(),
            sprite.region(),
            width,
            height,
        );
        if vertices.is_empty() {
            continue;
        }
        if let Some(batch) = batches
            .last_mut()
            .filter(|batch| batch.texture.shares_data_with(sprite.texture()))
        {
            batch.vertices.extend(vertices);
        } else {
            batches.push(TexturedSpriteBatch {
                texture: sprite.texture(),
                vertices,
            });
        }
    }
    batches
}

impl FrameGeometry {
    #[expect(
        clippy::cast_precision_loss,
        reason = "surface dimensions become f32 GPU clip-space coordinates"
    )]
    pub(crate) fn new(frame: &RenderFrame, width: u32, height: u32) -> Self {
        if width == 0 || height == 0 {
            return Self {
                vertices: Vec::new(),
            };
        }
        let camera = frame.camera();
        let viewport_height = camera.viewport_height();
        let mut vertices = Vec::with_capacity(frame.sprites().len() * 6);
        if viewport_height.is_finite() && viewport_height > 0.0 {
            let aspect = width as f32 / height as f32;
            let half_height = viewport_height * 0.5;
            let half_width = half_height * aspect;
            let center = camera.center();
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
                    vertex(left, top, color, [0.0, 0.0]),
                    vertex(left, bottom, color, [0.0, 1.0]),
                    vertex(right, bottom, color, [1.0, 1.0]),
                    vertex(left, top, color, [0.0, 0.0]),
                    vertex(right, bottom, color, [1.0, 1.0]),
                    vertex(right, top, color, [1.0, 0.0]),
                ]);
            }
        }
        if let Some(overlay) = frame.timing_overlay() {
            vertices.extend(timing_overlay_vertices(overlay));
        }
        vertices.extend(ui_vertices(frame.ui(), width, height));
        Self { vertices }
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "surface dimensions and bitmap coordinates become f32 GPU clip-space coordinates"
)]
fn ui_vertices(ui: &[super::UiPrimitive], width: u32, height: u32) -> Vec<SpriteVertex> {
    let mut vertices = Vec::new();
    for primitive in ui {
        match primitive {
            super::UiPrimitive::Rect(rect) => vertices.extend(screen_rect_vertices(
                rect.position(),
                rect.size(),
                rect.color().components(),
                width as f32,
                height as f32,
            )),
            super::UiPrimitive::Text(label) => {
                vertices.extend(text_vertices(label, width as f32, height as f32));
            }
        }
    }
    vertices
}

fn text_vertices(label: &super::TextLabel, width: f32, height: f32) -> Vec<SpriteVertex> {
    let origin = label.position();
    let scale = label.pixel_scale();
    let color = label.color().components();
    let mut cursor = origin;
    let mut vertices = Vec::new();
    for character in label.text().chars() {
        if character == '\n' {
            cursor[0] = origin[0];
            cursor[1] += 8.0 * scale;
            continue;
        }
        let rows = super::font::glyph_rows(character);
        let mut y = cursor[1];
        for bits in rows {
            let mut x = cursor[0];
            for column in 0..5 {
                if bits & (1 << (4 - column)) != 0 {
                    vertices.extend(screen_rect_vertices(
                        [x, y],
                        [scale, scale],
                        color,
                        width,
                        height,
                    ));
                }
                x += scale;
            }
            y += scale;
        }
        cursor[0] += 6.0 * scale;
    }
    vertices
}

fn screen_rect_vertices(
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    width: f32,
    height: f32,
) -> [SpriteVertex; 6] {
    let left = position[0] / width * 2.0 - 1.0;
    let right = (position[0] + size[0]) / width * 2.0 - 1.0;
    let top = 1.0 - position[1] / height * 2.0;
    let bottom = 1.0 - (position[1] + size[1]) / height * 2.0;
    [
        vertex(left, top, color, [0.0, 0.0]),
        vertex(left, bottom, color, [0.0, 0.0]),
        vertex(right, bottom, color, [0.0, 0.0]),
        vertex(left, top, color, [0.0, 0.0]),
        vertex(right, bottom, color, [0.0, 0.0]),
        vertex(right, top, color, [0.0, 0.0]),
    ]
}

#[expect(
    clippy::cast_precision_loss,
    reason = "small diagnostic counts become normalized GPU bar widths"
)]
fn timing_overlay_vertices(overlay: super::TimingOverlay) -> Vec<SpriteVertex> {
    let frame_ratio = (overlay.frame_elapsed().as_secs_f32() / (1.0 / 60.0)).clamp(0.0, 2.0) * 0.5;
    let fixed_ratio = ((overlay.fixed_steps() as f32) / 4.0).clamp(0.0, 1.0);
    let lag_ratio = (overlay.accumulated_lag().as_secs_f32() / 0.1).clamp(0.0, 1.0);
    let frame_color = if frame_ratio > 0.5 {
        [1.0, 0.72, 0.18, 0.95]
    } else {
        [0.2, 0.9, 0.45, 0.95]
    };
    let lag_color = if overlay.overloaded() {
        [1.0, 0.18, 0.16, 0.95]
    } else {
        [0.55, 0.65, 0.9, 0.95]
    };
    let mut vertices = Vec::with_capacity(18);
    vertices.extend(clip_rect(-0.96, 0.96, frame_ratio, frame_color));
    vertices.extend(clip_rect(-0.96, 0.90, fixed_ratio, [0.3, 0.65, 1.0, 0.95]));
    vertices.extend(clip_rect(-0.96, 0.84, lag_ratio, lag_color));
    vertices
}

fn clip_rect(left: f32, top: f32, ratio: f32, color: [f32; 4]) -> [SpriteVertex; 6] {
    let right = left + 0.42 * ratio;
    let bottom = top - 0.035;
    [
        vertex(left, top, color, [0.0, 0.0]),
        vertex(left, bottom, color, [0.0, 0.0]),
        vertex(right, bottom, color, [0.0, 0.0]),
        vertex(left, top, color, [0.0, 0.0]),
        vertex(right, bottom, color, [0.0, 0.0]),
        vertex(right, top, color, [0.0, 0.0]),
    ]
}

#[expect(
    clippy::cast_precision_loss,
    reason = "surface dimensions become f32 GPU clip-space coordinates"
)]
fn projected_vertices(
    camera: super::Camera2d,
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
    region: super::SpriteRegion,
    width: u32,
    height: u32,
) -> Vec<SpriteVertex> {
    let viewport_height = camera.viewport_height();
    if width == 0
        || height == 0
        || !viewport_height.is_finite()
        || viewport_height <= 0.0
        || !position.into_iter().all(f32::is_finite)
        || !size
            .into_iter()
            .all(|value| value.is_finite() && value > 0.0)
    {
        return Vec::new();
    }
    let half_height = viewport_height * 0.5;
    let half_width = half_height * (width as f32 / height as f32);
    let center = camera.center();
    let left = (position[0] - size[0] * 0.5 - center[0]) / half_width;
    let right = (position[0] + size[0] * 0.5 - center[0]) / half_width;
    let top = -(position[1] - size[1] * 0.5 - center[1]) / half_height;
    let bottom = -(position[1] + size[1] * 0.5 - center[1]) / half_height;
    let uv_min = region.min();
    let uv_max = region.max();
    vec![
        vertex(left, top, color, uv_min),
        vertex(left, bottom, color, [uv_min[0], uv_max[1]]),
        vertex(right, bottom, color, uv_max),
        vertex(left, top, color, uv_min),
        vertex(right, bottom, color, uv_max),
        vertex(right, top, color, [uv_max[0], uv_min[1]]),
    ]
}

fn vertex(x: f32, y: f32, color: [f32; 4], uv: [f32; 2]) -> SpriteVertex {
    SpriteVertex {
        position: [x, y],
        color,
        uv,
    }
}

#[cfg(test)]
mod test;
