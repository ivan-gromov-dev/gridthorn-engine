use crate::{RandomStreamError, RandomStreams};

#[test]
fn registration_order_and_unrelated_consumption_do_not_change_streams() {
    let mut first = RandomStreams::new(42);
    first.register("economy").unwrap();
    first.register("weather").unwrap();
    let mut second = RandomStreams::new(42);
    second.register("weather").unwrap();
    second.register("economy").unwrap();
    assert_eq!(first, second);
    for _ in 0..10 {
        first.next_u64("weather").unwrap();
        assert_eq!(
            first.next_u64("economy").unwrap(),
            second.next_u64("economy").unwrap()
        );
    }
    assert_eq!(
        first.states().map(|(name, _)| name).collect::<Vec<_>>(),
        vec!["economy", "weather"]
    );
}

#[test]
fn validation_errors_do_not_change_any_stream() {
    let mut streams = RandomStreams::new(42);
    streams.register("economy").unwrap();
    let before = streams.clone();
    assert_eq!(
        streams.register("economy"),
        Err(RandomStreamError::Duplicate("economy".into()))
    );
    assert_eq!(streams.register(""), Err(RandomStreamError::InvalidName));
    assert_eq!(
        streams.register(" padded "),
        Err(RandomStreamError::InvalidName)
    );
    assert_eq!(
        streams.next_u64("unknown"),
        Err(RandomStreamError::Missing("unknown".into()))
    );
    assert_eq!(streams, before);
}

#[test]
fn cloning_restores_exact_future_consumption() {
    let mut streams = RandomStreams::new(42);
    streams.register("economy").unwrap();
    streams.next_u64("economy").unwrap();
    let mut snapshot = streams.clone();
    for _ in 0..32 {
        assert_eq!(
            streams.next_u64("economy").unwrap(),
            snapshot.next_u64("economy").unwrap()
        );
    }
}

#[test]
fn named_seed_encoding_and_consumption_match_fixed_vectors() {
    let mut streams = RandomStreams::new(42);
    streams.register("economy").unwrap();
    assert_eq!(
        streams.states().collect::<Vec<_>>(),
        vec![("economy", 0x21b8_2d29_fc05_3710)]
    );
    assert_eq!(streams.next_u64("economy").unwrap(), 0x3ee9_6a6f_ef43_c8f3);
    assert_eq!(streams.next_u64("economy").unwrap(), 0x4747_29ad_eb10_0b19);
    assert_eq!(streams.next_u64("economy").unwrap(), 0xb776_a43f_6915_ee7e);
}
#[test]
fn persisted_positions_preserve_future_values_and_reject_duplicate_names() {
    let mut original = super::super::RandomStreams::new(u64::MAX);
    original.register("economy").unwrap();
    original.next_u64("economy").unwrap();
    let states = original
        .states()
        .map(|(name, state)| (name.to_owned(), state))
        .collect::<Vec<_>>();
    let mut restored =
        super::super::RandomStreams::from_states(original.seed(), states.clone()).unwrap();
    assert_eq!(original, restored);
    assert_eq!(original.next_u64("economy"), restored.next_u64("economy"));
    assert!(
        super::super::RandomStreams::from_states(0, states.clone().into_iter().chain(states))
            .is_err()
    );
    assert!(super::super::RandomStreams::from_states(0, [(" padded".to_owned(), 0)]).is_err());
}
