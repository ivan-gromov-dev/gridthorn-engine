use super::*;

#[test]
fn overflow_keeps_complete_frames_and_reports_drops() {
    let buffer = CaptureBuffer {
        samples: Mutex::new(VecDeque::with_capacity(4)),
        capacity: 4,
        dropped: AtomicU64::new(0),
    };
    buffer.push(&[1., 2., 3., 4., 5., 6.], 2);
    assert_eq!(
        *buffer.samples.lock().unwrap(),
        VecDeque::from([1., 2., 3., 4.])
    );
    assert_eq!(buffer.dropped.load(Ordering::Relaxed), 1);
    buffer.push(&[7., 8.], 2);
    assert_eq!(buffer.dropped.load(Ordering::Relaxed), 2);
}

#[test]
fn callback_does_not_block_when_consumer_holds_buffer() {
    let buffer = CaptureBuffer {
        samples: Mutex::new(VecDeque::with_capacity(4)),
        capacity: 4,
        dropped: AtomicU64::new(0),
    };
    let _guard = buffer.samples.lock().unwrap();
    buffer.push(&[0.; 4], 2);
    assert_eq!(buffer.dropped.load(Ordering::Relaxed), 2);
}

#[test]
fn invalid_capacity_is_rejected_before_device_access() {
    for capacity in [0, 1_048_577] {
        assert!(matches!(
            AudioCapture::start(AudioDeviceSelection::Default, capacity),
            Err(AudioDeviceError::InvalidCapacity)
        ));
    }
}

#[test]
#[ignore = "opens default capture briefly; samples are discarded without persistence"]
fn native_capture_drains_and_switches() {
    let mut capture = AudioCapture::start(AudioDeviceSelection::Default, 4096).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let batch = capture.drain().unwrap();
    assert!(batch.channels > 0);
    assert!(batch.sample_rate > 0);
    assert_ne!(batch.samples, []);
    assert_eq!(batch.samples.len() % usize::from(batch.channels), 0);
    assert!(batch.samples.len() <= 4096 * usize::from(batch.channels));
    let id = capture.device().clone();
    capture
        .select_device(AudioDeviceSelection::Device(id.clone()))
        .unwrap();
    assert_eq!(capture.device(), &id);
    capture.failed.store(true, Ordering::Release);
    assert!(capture.drain().is_err());
    assert!(capture.refresh_device().unwrap());
    println!(
        "capture diagnostic after reconnect: {:?}",
        capture.take_device_error()
    );
    assert!(!capture.refresh_device().unwrap());
    capture
        .select_device(AudioDeviceSelection::Default)
        .unwrap();
}
