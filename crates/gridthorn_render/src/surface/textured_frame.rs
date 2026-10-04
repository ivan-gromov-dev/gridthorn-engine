use super::lifecycle::SurfaceExtent;
use crate::{Camera2d, RenderFrame, TexturedSprite};

/// Compare decoded identities rather than scanning atlas pixels for each sprite.
pub(super) struct TexturedFrame {
    previous: Option<(Camera2d, SurfaceExtent, Vec<TexturedSprite>)>,
}

impl TexturedFrame {
    pub fn new() -> Self {
        Self { previous: None }
    }

    #[expect(
        clippy::float_cmp,
        reason = "any changed GPU input must invalidate retained geometry"
    )]
    pub fn prepare(&mut self, frame: &RenderFrame, extent: SurfaceExtent) -> bool {
        if self
            .previous
            .as_ref()
            .is_some_and(|(camera, size, sprites)| {
                *camera == frame.camera()
                    && *size == extent
                    && sprites.len() == frame.textured_sprites().len()
                    && sprites
                        .iter()
                        .zip(frame.textured_sprites())
                        .all(|(left, right)| {
                            left.position() == right.position()
                                && left.size() == right.size()
                                && left.tint() == right.tint()
                                && left.region() == right.region()
                                && left.texture().shares_data_with(right.texture())
                        })
            })
        {
            return false;
        }
        self.previous = Some((frame.camera(), extent, frame.textured_sprites().to_vec()));
        true
    }
}

#[cfg(test)]
#[path = "textured_frame/test/mod.rs"]
mod test;
