use super::super::{native::NativePresentation, *};

#[test]
fn renderer_free_request_reports_unavailable_and_preserves_cap() {
    let mut native = NativePresentation::default();
    let config = PresentationConfig {
        frame_rate_limit: Some(FrameRateLimit::new(30).unwrap()),
        ..PresentationConfig::default()
    };
    assert_eq!(
        native.configure(None, 4, config),
        PresentationOperation::Failed {
            id: 4,
            error: PresentationError::Unavailable
        }
    );
    assert_eq!(native.pacer.limit, None);
    let (state, feedback) = native.observe(None);
    assert_eq!(state, Some(PresentationState::default()));
    assert_eq!(feedback, None);
    assert_eq!(native.observe(None), (None, None));
}
