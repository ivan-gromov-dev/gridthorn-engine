use super::{TextLabel, UiError, UiRect};
use crate::Color;

#[test]
fn rejects_invalid_screen_geometry() {
    assert_eq!(
        UiRect::new([0.0, 0.0], [0.0, 10.0], Color::default()),
        Err(UiError::InvalidSize)
    );
    assert_eq!(
        TextLabel::new("label", [f32::NAN, 0.0], 1.0, Color::default()),
        Err(UiError::InvalidPosition)
    );
    assert_eq!(
        TextLabel::new("label", [0.0, 0.0], 0.0, Color::default()),
        Err(UiError::InvalidPixelScale)
    );
}
