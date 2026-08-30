use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use gridthorn_assets::TextureAsset;

use super::super::textured_sprite_batches;
use crate::{Camera2d, RenderFrame, SpriteRegion, TexturedSprite};

#[test]
fn combines_adjacent_sprites_that_share_one_texture_handle() {
    let texture = load_texture("shared", [255, 0, 0]);
    let frame = RenderFrame::new(Camera2d::default(), Vec::new()).with_textured_sprites(vec![
        TexturedSprite::new([0.0, 0.0], [8.0, 8.0], texture.clone()),
        TexturedSprite::new([10.0, 0.0], [8.0, 8.0], texture),
    ]);

    let batches = textured_sprite_batches(&frame, 800, 600);

    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].vertices.len(), 12);
}

#[test]
fn maps_sprite_sheet_regions_into_batch_uv_coordinates() {
    let region = SpriteRegion::new([0.25, 0.5], [0.5, 1.0]).expect("region should be valid");
    let frame = RenderFrame::new(Camera2d::default(), Vec::new()).with_textured_sprites(vec![
        TexturedSprite::new(
            [0.0, 0.0],
            [8.0, 8.0],
            load_texture("region", [255, 255, 255]),
        )
        .with_region(region),
    ]);

    let batches = textured_sprite_batches(&frame, 800, 600);

    assert_slice_close(&batches[0].vertices[0].uv, &[0.25, 0.5]);
    assert_slice_close(&batches[0].vertices[2].uv, &[0.5, 1.0]);
    assert_slice_close(&batches[0].vertices[5].uv, &[0.5, 0.5]);
}

#[test]
fn preserves_submission_order_across_texture_changes() {
    let first = load_texture("first", [255, 0, 0]);
    let second = load_texture("second", [0, 255, 0]);
    let frame = RenderFrame::new(Camera2d::default(), Vec::new()).with_textured_sprites(vec![
        TexturedSprite::new([0.0, 0.0], [8.0, 8.0], first.clone()),
        TexturedSprite::new([10.0, 0.0], [8.0, 8.0], second),
        TexturedSprite::new([20.0, 0.0], [8.0, 8.0], first),
    ]);

    let batches = textured_sprite_batches(&frame, 800, 600);

    assert_eq!(batches.len(), 3);
    assert!(batches[0].texture.rgba8().starts_with(&[255, 0, 0]));
    assert!(batches[1].texture.rgba8().starts_with(&[0, 255, 0]));
    assert!(batches[2].texture.rgba8().starts_with(&[255, 0, 0]));
}

fn load_texture(label: &str, rgb: [u8; 3]) -> TextureAsset {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("gridthorn-batch-{label}-{unique}.ppm"));
    let mut fixture = b"P6\n1 1\n255\n".to_vec();
    fixture.extend(rgb);
    fs::write(&path, fixture).expect("write texture fixture");
    let texture = TextureAsset::load(&path).expect("load texture fixture");
    fs::remove_file(path).expect("remove texture fixture");
    texture
}

fn assert_slice_close(actual: &[f32], expected: &[f32]) {
    assert!(
        actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
}
