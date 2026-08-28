use crate::StateFingerprint;

#[test]
fn fnv1a_matches_fixed_vector() {
    let mut fingerprint = StateFingerprint::new();
    fingerprint.write_bytes(b"hello");

    assert_eq!(fingerprint.finish(), 0xa430_d846_80aa_bd0b);
}

#[test]
fn integer_encoding_is_explicitly_little_endian() {
    let mut encoded = StateFingerprint::new();
    encoded.write_u64(0x0807_0605_0403_0201);

    let mut bytes = StateFingerprint::new();
    bytes.write_bytes(&[1, 2, 3, 4, 5, 6, 7, 8]);

    assert_eq!(encoded.finish(), bytes.finish());
}
