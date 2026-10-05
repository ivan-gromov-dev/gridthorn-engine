use super::*;

#[test]
fn pass_budget_rejects_each_limit_atomically_and_fresh_pass_starts_empty() {
    let limits = [ENTRY_LIMIT, KEY_BYTES_LIMIT, GLYPH_LIMIT, LINE_LIMIT];
    for index in 0..4 {
        let mut pass = PreparedText::new(None);
        let mut full = [1; 4];
        full[index] = limits[index];
        assert!(pass.retain(full));
        assert!(!pass.retain([1; 4]));
        assert_eq!(pass.retained, full);
        let mut oversized = [0; 4];
        oversized[index] = usize::MAX;
        assert!(!pass.retain(oversized));
        assert_eq!(pass.retained, full);
    }
    let mut fresh = PreparedText::new(None);
    assert_eq!(fresh.retained, [0; 4]);
    assert!(fresh.retain([1; 4]));
    assert!(
        fresh
            .layout("caption", &TextStyle::new("Noto Sans", 20.0))
            .is_err()
    );
    assert!(fresh.layouts.is_empty());
}
