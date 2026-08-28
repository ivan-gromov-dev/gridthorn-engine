use crate::DeterministicRng;

#[test]
fn splitmix64_matches_fixed_vectors() {
    let mut rng = DeterministicRng::new(0);

    assert_eq!(rng.next_u64(), 0xe220_a839_7b1d_cdaf);
    assert_eq!(rng.next_u64(), 0x6e78_9e6a_a1b9_65f4);
    assert_eq!(rng.next_u64(), 0x06c4_5d18_8009_454f);
}

#[test]
fn equal_seeds_produce_equal_streams() {
    let mut left = DeterministicRng::new(42);
    let mut right = DeterministicRng::new(42);

    for _ in 0..32 {
        assert_eq!(left.next_u64(), right.next_u64());
    }
}
