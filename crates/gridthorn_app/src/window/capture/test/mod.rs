use super::*;

#[test]
fn failed_capture_retains_effective_mode_and_reports_context() {
    let mut capture = NativeCapture::default();
    let applied = capture.apply(PointerCaptureMode::Confined, true, |_| Ok(()));
    assert_eq!(applied.effective, PointerCaptureMode::Confined);
    let failed = capture.apply(PointerCaptureMode::Locked, true, |_| {
        Err("unsupported".into())
    });
    assert_eq!(failed.effective, PointerCaptureMode::Confined);
    assert!(
        failed
            .error
            .unwrap()
            .to_string()
            .contains("Locked failed: unsupported")
    );
    let cancelled = capture.apply(PointerCaptureMode::None, false, |_| Ok(()));
    assert_eq!(cancelled.effective, PointerCaptureMode::None);
}

#[test]
fn unfocused_capture_does_not_call_platform() {
    let mut capture = NativeCapture::default();
    let status = capture.apply(PointerCaptureMode::Locked, false, |_| {
        panic!("unfocused request")
    });
    assert_eq!(status.effective, PointerCaptureMode::None);
    assert!(status.error.unwrap().to_string().contains("focused window"));
}
