use super::super::*;
use crate::display::{DisplayResolution, MonitorId};

fn size(width: u32, height: u32) -> DisplayResolution {
    DisplayResolution { width, height }
}

#[test]
fn rejects_invalid_requests_without_overwriting_queued_operation() {
    let mut settings = WindowSettings::default();
    assert!(!settings.capabilities().available);
    let request = WindowRequest {
        size: Some(size(800, 600)),
        ..WindowRequest::default()
    };
    assert_eq!(settings.request(request), Ok(1));
    for invalid in [
        WindowRequest::default(),
        WindowRequest {
            size: Some(size(0, 600)),
            ..request
        },
        WindowRequest {
            resize_policy: Some(WindowResizePolicy::Resizable {
                min: Some(size(900, 600)),
                max: Some(size(800, 600)),
            }),
            ..request
        },
        WindowRequest {
            mode: Some(WindowMode::Borderless {
                monitor: MonitorId(1, 1),
            }),
            ..request
        },
    ] {
        assert!(settings.request(invalid).is_err());
    }
    assert_eq!(
        settings.feedback(),
        Some(&WindowOperation::Pending { id: 1 })
    );
    assert_eq!(settings.take_request(), Some((1, request)));
    assert_eq!(settings.take_request(), None);
}

#[test]
fn latest_request_wins_and_old_feedback_cannot_replace_it() {
    let mut settings = WindowSettings::default();
    settings
        .request(WindowRequest {
            size: Some(size(800, 600)),
            ..WindowRequest::default()
        })
        .unwrap();
    let second = WindowRequest {
        resize_policy: Some(WindowResizePolicy::Fixed),
        ..WindowRequest::default()
    };
    settings.request(second).unwrap();
    settings.publish_feedback(WindowOperation::Pending { id: 1 });
    assert_eq!(
        settings.feedback(),
        Some(&WindowOperation::Pending { id: 2 })
    );
    assert_eq!(settings.take_request(), Some((2, second)));
}
