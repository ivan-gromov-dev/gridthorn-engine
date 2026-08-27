use super::validate_engine_requirement;

#[test]
fn rejects_incompatible_engine_requirement() {
    let error = validate_engine_requirement(">=99.0.0")
        .expect_err("a future engine requirement must be rejected");
    assert!(error.to_string().contains("this CLI is"));
}
