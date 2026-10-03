use super::timestamp_duration;
use std::time::Duration;

#[test]
fn timestamps_apply_backend_period_and_reject_invalid_readbacks() {
    let bytes = [100_u64.to_le_bytes(), 2100_u64.to_le_bytes()].concat();
    assert_eq!(
        timestamp_duration(&bytes, 2.0),
        Some(Duration::from_micros(4))
    );
    assert_eq!(timestamp_duration(&bytes[..8], 2.0), None);
    let reversed = [2100_u64.to_le_bytes(), 100_u64.to_le_bytes()].concat();
    assert_eq!(timestamp_duration(&reversed, 2.0), None);
    assert_eq!(timestamp_duration(&bytes, f32::NAN), None);
    assert_eq!(timestamp_duration(&bytes, 0.0), None);
}
