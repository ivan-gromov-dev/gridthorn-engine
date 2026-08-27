use super::validate;

#[test]
fn accepts_cargo_compatible_project_names() {
    assert!(validate("valid-name_2").is_ok());
}

#[test]
fn rejects_invalid_project_names() {
    assert!(validate("9lives").is_err());
    assert!(validate("has spaces").is_err());
}
