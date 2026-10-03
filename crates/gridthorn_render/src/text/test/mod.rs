mod geometry;
mod layout;
mod raster;

use super::{TextStyle, TextSystem};
use gridthorn_assets::FontAsset;

fn assets() -> Vec<FontAsset> {
    [
        include_bytes!("fonts/NotoSans-Regular.ttf").as_slice(),
        include_bytes!("fonts/NotoSansArabic-Regular.ttf").as_slice(),
        include_bytes!("fonts/NotoSansJP-Regular.otf").as_slice(),
    ]
    .into_iter()
    .map(|bytes| FontAsset::from_bytes(bytes.to_vec()).expect("valid fixture font"))
    .collect()
}

pub(super) fn system() -> TextSystem {
    TextSystem::new("en-US", &assets()).expect("font service")
}
pub(super) fn style() -> TextStyle {
    TextStyle::new("Noto Sans", 24.0)
}
