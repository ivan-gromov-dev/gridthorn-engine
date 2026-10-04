use super::*;
use crate::{Camera2d, TexturedSprite};

fn texture() -> TextureAsset {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("gridthorn-gpu-cache-{unique}.ppm"));
    let mut pixels = b"P6\n256 256\n255\n".to_vec();
    pixels.resize(pixels.len() + 256 * 256 * 3, 255);
    std::fs::write(&path, pixels).unwrap();
    let asset = TextureAsset::load(&path).unwrap();
    std::fs::remove_file(path).unwrap();
    asset
}

#[test]
#[ignore = "requires a native GPU adapter; release resource scaling probe"]
fn measure_native_texture_reuse_and_dirty_geometry() {
    let adapter = pollster::block_on(
        wgpu::Instance::default().request_adapter(&wgpu::RequestAdapterOptions::default()),
    )
    .unwrap();
    let (device, queue) =
        pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();
    let layout = super::super::pipeline::sampled_texture_layout(&device);
    let first = texture();
    let second = texture();
    let extent = SurfaceExtent {
        width: 1000,
        height: 800,
    };
    let mut resources = TexturedResources::new();
    for count in [1, 32, 1024] {
        let frame = RenderFrame::default().with_textured_sprites(
            (0..count)
                .map(|_| TexturedSprite::new([0.0; 2], [8.0; 2], first.clone()))
                .collect(),
        );
        resources.prepare(&device, &queue, &layout, &RenderFrame::default(), extent);
        resources.prepare(&device, &queue, &layout, &frame, extent);
        assert_eq!(resources.uploaded_texture_bytes, 256 * 256 * 4);
        assert_eq!(resources.batches.len(), 1);
        assert_eq!(resources.batches[0].2 as usize, count * 6);
        for dirty in [false, true] {
            let mut samples = Vec::new();
            for index in 0..110 {
                let camera = if dirty {
                    Camera2d::new([if index % 2 == 0 { 1.0 } else { 2.0 }; 2], 300.0)
                } else {
                    Camera2d::default()
                };
                let changed = RenderFrame::new(camera, Vec::new())
                    .with_textured_sprites(frame.textured_sprites().to_vec());
                let start = std::time::Instant::now();
                resources.prepare(&device, &queue, &layout, &changed, extent);
                let elapsed = start.elapsed();
                assert_eq!(resources.uploaded_texture_bytes, 0);
                assert_eq!(resources.changed, dirty);
                if index >= 10 {
                    samples.push(elapsed.as_nanos());
                }
            }
            samples.sort_unstable();
            eprintln!(
                "texture_resource_probe,{count},{dirty},100,{},{},{},{}",
                samples[49], samples[94], samples[98], samples[99]
            );
        }
    }
    let ordered = RenderFrame::default().with_textured_sprites(vec![
        TexturedSprite::new([0.0; 2], [8.0; 2], first.clone()),
        TexturedSprite::new([1.0; 2], [8.0; 2], second.clone()),
        TexturedSprite::new([2.0; 2], [8.0; 2], first),
    ]);
    resources.prepare(&device, &queue, &layout, &ordered, extent);
    assert_eq!(resources.batches.len(), 3);
    assert_eq!(resources.texture_count(), 2);
    assert_eq!(resources.uploaded_texture_bytes, second.rgba8().len());
    let reloaded = RenderFrame::default().with_textured_sprites(vec![TexturedSprite::new(
        [0.0; 2],
        [8.0; 2],
        texture(),
    )]);
    resources.prepare(&device, &queue, &layout, &reloaded, extent);
    assert_eq!(resources.texture_count(), 1);
    assert_eq!(resources.uploaded_texture_bytes, 256 * 256 * 4);
    resources.prepare(&device, &queue, &layout, &RenderFrame::default(), extent);
    assert_eq!(resources.texture_count(), 0);
    assert_eq!(resources.texture_bytes(), 0);
    assert_eq!(resources.batches.len(), 0);
}
