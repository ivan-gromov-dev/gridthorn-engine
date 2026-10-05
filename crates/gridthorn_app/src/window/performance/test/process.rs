use super::*;

#[test]
fn cpu_work_is_independent_of_wall_waits_and_can_span_parallel_threads() {
    let mut samples = ProcessSamples::default();
    samples.record(Some(Duration::from_millis(10)), Duration::from_millis(100));
    samples.record(Some(Duration::from_millis(12)), Duration::from_millis(200));
    samples.record(Some(Duration::from_millis(42)), Duration::from_millis(210));
    assert_eq!(samples.samples[0].1, Duration::from_millis(2));
    assert_eq!(samples.samples[0].2, Duration::from_millis(100));
    assert_eq!(samples.samples[1].1, Duration::from_millis(30));
    assert_eq!(samples.samples[1].2, Duration::from_millis(10));
}

#[test]
fn failed_reads_do_not_merge_multiple_frames_into_one_sample() {
    let mut samples = ProcessSamples::default();
    samples.record(Some(Duration::from_millis(10)), Duration::from_millis(100));
    samples.record(None, Duration::from_millis(200));
    samples.record(Some(Duration::from_millis(30)), Duration::from_millis(300));
    assert_eq!(samples.samples.len(), 0);
    samples.record(Some(Duration::from_millis(31)), Duration::from_millis(400));
    assert_eq!(samples.samples[0].0, 4);
    assert_eq!(samples.samples[0].1, Duration::from_millis(1));
    assert_eq!(samples.errors, 1);
}

#[test]
fn collection_is_bounded_and_rejects_regressing_clocks() {
    let mut samples = ProcessSamples::default();
    samples.record(Some(Duration::from_secs(2)), Duration::from_secs(2));
    samples.record(Some(Duration::from_secs(1)), Duration::from_secs(1));
    assert_eq!(samples.samples.len(), 0);
    for index in 2..LIMIT + 12 {
        let time = Duration::from_secs(index as u64);
        samples.record(Some(time), time);
    }
    assert_eq!(samples.samples.len(), LIMIT);
    assert_eq!(samples.frames, LIMIT as u64 + 2);
}
