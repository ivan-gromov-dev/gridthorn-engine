use super::version;

#[test]
fn reports_package_version() {
    assert_eq!(version(), env!("CARGO_PKG_VERSION"));
}
