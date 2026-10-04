use super::*;
use crate::{Color, SpriteRegion};
use gridthorn_assets::TextureAsset;

fn texture() -> TextureAsset {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("gridthorn-textured-frame-{unique}.ppm"));
    std::fs::write(&path, b"P6\n1 1\n255\n\xff\x00\x00").unwrap();
    let asset = TextureAsset::load(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    asset
}

#[test]
fn cache_invalidates_for_geometry_order_camera_extent_and_reloaded_identity() {
    let asset = texture();
    let sprite = TexturedSprite::new([0.0; 2], [8.0; 2], asset.clone());
    let other = TexturedSprite::new([0.0; 2], [8.0; 2], texture());
    let extent = SurfaceExtent {
        width: 800,
        height: 600,
    };
    let variants = [
        vec![sprite.clone()],
        vec![TexturedSprite::new([1.0; 2], [8.0; 2], asset.clone())],
        vec![TexturedSprite::new([1.0; 2], [9.0; 2], asset.clone())],
        vec![sprite.clone().with_tint(Color::rgb(0.0, 1.0, 0.0))],
        vec![
            sprite
                .clone()
                .with_region(SpriteRegion::new([0.0; 2], [0.5; 2]).unwrap()),
        ],
        vec![sprite.clone(), other.clone()],
        vec![other, sprite],
        Vec::new(),
    ];
    let mut cache = TexturedFrame::new();
    for sprites in variants {
        let frame = RenderFrame::default().with_textured_sprites(sprites);
        assert!(cache.prepare(&frame, extent));
        assert!(!cache.prepare(&frame.clone(), extent));
    }
    let frame = RenderFrame::default()
        .with_textured_sprites(vec![TexturedSprite::new([0.0; 2], [8.0; 2], asset)]);
    assert!(cache.prepare(&frame, extent));
    assert!(cache.prepare(
        &frame,
        SurfaceExtent {
            width: 801,
            ..extent
        }
    ));
    let moved = RenderFrame::new(Camera2d::new([1.0; 2], 300.0), Vec::new())
        .with_textured_sprites(frame.textured_sprites().to_vec());
    assert!(cache.prepare(&moved, extent));
    assert!(cache.prepare(&frame, extent));
    assert!(cache.prepare(
        &RenderFrame::default().with_textured_sprites(vec![TexturedSprite::new(
            [0.0; 2],
            [8.0; 2],
            texture()
        )]),
        extent
    ));
}
