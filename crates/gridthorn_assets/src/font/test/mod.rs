use super::{FontAsset, FontAssetError};

#[test]
fn rejects_invalid_and_oversized_fonts() {
    assert!(matches!(
        FontAsset::from_bytes(vec![0; 32]),
        Err(FontAssetError::InvalidFont)
    ));
    assert!(matches!(
        FontAsset::from_bytes(vec![0; 32 * 1024 * 1024 + 1]),
        Err(FontAssetError::TooLarge)
    ));
}

#[test]
fn reports_missing_file_path() {
    let path = std::env::temp_dir().join("gridthorn-missing-font-fixture.ttf");
    assert!(
        matches!(FontAsset::load(&path), Err(FontAssetError::Read { path: actual, .. }) if actual == path)
    );
}
