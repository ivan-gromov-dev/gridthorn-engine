use super::*;

#[test]
fn exhausted_identifiers_preserve_queued_request_and_feedback() {
    let config = PresentationConfig::default();
    let mut settings = PresentationSettings {
        next_id: u64::MAX,
        request: Some((7, config)),
        feedback: Some(PresentationOperation::Pending { id: 7 }),
        ..PresentationSettings::default()
    };
    assert_eq!(
        settings.request(config),
        Err(PresentationError::RequestIdsExhausted)
    );
    assert_eq!(settings.take_request(), Some((7, config)));
    assert_eq!(
        settings.feedback(),
        Some(&PresentationOperation::Pending { id: 7 })
    );
}
