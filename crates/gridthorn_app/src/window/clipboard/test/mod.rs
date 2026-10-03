use super::{execute_with, map_error};
use gridthorn_input::{ClipboardError, ClipboardOperation, ClipboardRequest};

#[test]
fn unfocused_requests_never_access_the_backend() {
    let request = ClipboardRequest {
        id: 7,
        operation: ClipboardOperation::Read,
    };
    let response = execute_with(&request, false, |_| panic!("unfocused backend access"));
    assert_eq!(response.id, 7);
    assert_eq!(response.result, Err(ClipboardError::Unfocused));
}

#[test]
fn responses_preserve_unicode_and_recoverable_errors() {
    let request = ClipboardRequest {
        id: 9,
        operation: ClipboardOperation::Read,
    };
    assert_eq!(
        execute_with(&request, true, |_| Ok(Some("Привет 日本 👋".into()))).result,
        Ok(Some("Привет 日本 👋".into()))
    );
    assert_eq!(
        map_error("read", arboard::Error::ContentNotAvailable),
        ClipboardError::NoText
    );
    assert!(
        map_error("initialize", arboard::Error::ClipboardNotSupported)
            .to_string()
            .contains("initialize")
    );
    assert_eq!(
        execute_with(&request, true, |_| Err(ClipboardError::NoText)).result,
        Err(ClipboardError::NoText)
    );
}
#[test]
#[ignore = "requires a desktop clipboard containing restorable text"]
fn native_unicode_clipboard_round_trip_restores_original_text() {
    let mut clipboard = super::NativeClipboard::default();
    let read = ClipboardRequest {
        id: 1,
        operation: ClipboardOperation::Read,
    };
    let original = clipboard
        .execute(&read, true)
        .result
        .expect("read original clipboard text")
        .expect("read returns text");
    let text = "Gridthorn Привет 日本 e\u{301} 👩‍💻";
    let write = clipboard.execute(
        &ClipboardRequest {
            id: 2,
            operation: ClipboardOperation::Write(text.into()),
        },
        true,
    );
    let read_back = clipboard.execute(&read, true);
    let restore = clipboard.execute(
        &ClipboardRequest {
            id: 3,
            operation: ClipboardOperation::Write(original.clone()),
        },
        true,
    );
    let restored = clipboard.execute(&read, true);
    assert_eq!(restore.result, Ok(None), "restore original clipboard");
    assert_eq!(
        restored.result,
        Ok(Some(original)),
        "verify clipboard restoration"
    );
    assert_eq!(write.result, Ok(None));
    assert_eq!(read_back.result, Ok(Some(text.into())));
}
