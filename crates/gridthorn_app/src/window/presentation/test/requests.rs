use super::super::*;

#[test]
fn coalesces_requests_and_preserves_newer_feedback_and_observed_state() {
    let mut settings = PresentationSettings::default();
    let config = PresentationConfig {
        present_mode: PresentMode::Mailbox,
        frame_rate_limit: Some(FrameRateLimit::new(144).unwrap()),
    };
    assert_eq!(settings.state().supported_modes, []);
    settings.request(PresentationConfig::default()).unwrap();
    let id = settings.request(config).unwrap();
    assert_eq!(settings.take_request(), Some((id, config)));
    assert_eq!(settings.take_request(), None);
    settings.publish_feedback(PresentationOperation::Applied {
        id: id - 1,
        config: PresentationConfig::default(),
    });
    assert_eq!(
        settings.feedback(),
        Some(&PresentationOperation::Pending { id })
    );
    assert_eq!(settings.state().applied_mode, None);
    settings.publish_feedback(PresentationOperation::Failed {
        id,
        error: PresentationError::UnsupportedMode(config.present_mode),
    });
    assert!(matches!(
        settings.feedback(),
        Some(PresentationOperation::Failed { .. })
    ));
}
