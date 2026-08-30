use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{TextureAsset, TextureAssetError};

#[test]
fn loads_rgba_pixels_from_a_portable_pixmap() {
    let path = temporary_path("valid", "ppm");
    fs::write(&path, b"P6\n2 1\n255\n\xff\x00\x00\x00\xff\x00").expect("write fixture");

    let texture = TextureAsset::load(&path).expect("decode fixture");

    assert_eq!(texture.dimensions(), [2, 1]);
    assert_eq!(texture.rgba8(), &[255, 0, 0, 255, 0, 255, 0, 255]);
    fs::remove_file(path).expect("remove fixture");
}

#[test]
fn reports_the_source_path_for_invalid_image_data() {
    let path = temporary_path("invalid", "png");
    fs::write(&path, b"not an image").expect("write fixture");

    let error = TextureAsset::load(&path).expect_err("invalid image must fail");

    assert!(matches!(error, TextureAssetError::Decode { .. }));
    assert!(error.to_string().contains(&path.display().to_string()));
    fs::remove_file(path).expect("remove fixture");
}

#[test]
fn cloned_handles_share_data_but_separate_loads_remain_distinct() {
    let path = temporary_path("identity", "ppm");
    fs::write(&path, b"P6\n1 1\n255\n\xff\x00\x00").expect("write fixture");
    let first = TextureAsset::load(&path).expect("decode first handle");
    let clone = first.clone();
    let second = TextureAsset::load(&path).expect("decode second handle");

    assert!(first.shares_data_with(&clone));
    assert!(!first.shares_data_with(&second));
    fs::remove_file(path).expect("remove fixture");
}

fn temporary_path(label: &str, extension: &str) -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!("gridthorn-{label}-{unique}.{extension}"))
}
