use crate::{Clipboard, ClipboardOperation};

#[test]
fn requests_preserve_order_unicode_and_identities() {
    let mut clipboard = Clipboard::default();
    clipboard.write(41, "Привет 👋");
    clipboard.read(42);
    let requests = clipboard.take_requests();
    assert_eq!(requests[0].id, 41);
    assert_eq!(
        requests[0].operation,
        ClipboardOperation::Write("Привет 👋".into())
    );
    assert_eq!(requests[1].id, 42);
    assert_eq!(requests[1].operation, ClipboardOperation::Read);
    assert_eq!(clipboard.take_requests(), []);
}
