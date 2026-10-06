use crate::presentation::{FrameRateLimit, PresentMode, PresentationConfig, PresentationSettings};

#[test]
fn game_can_request_vsync_and_independent_frame_cap_through_facade() {
    let mut settings = PresentationSettings::default();
    let config = PresentationConfig {
        present_mode: PresentMode::Fifo,
        frame_rate_limit: Some(FrameRateLimit::new(90).unwrap()),
    };
    assert_eq!(settings.request(config).unwrap(), 1);
    assert_eq!(config.frame_rate_limit.unwrap().fps(), 90);
}
