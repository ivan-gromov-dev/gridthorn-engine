use gridthorn_input::{ClipboardError, ClipboardOperation, ClipboardRequest, ClipboardResponse};

/// Event-loop-owned clipboard, initialized lazily and retained for Linux ownership.
#[derive(Default)]
pub(super) struct NativeClipboard {
    clipboard: Option<arboard::Clipboard>,
}

impl NativeClipboard {
    pub(super) fn execute(
        &mut self,
        request: &ClipboardRequest,
        focused: bool,
    ) -> ClipboardResponse {
        execute_with(request, focused, |operation| {
            if self.clipboard.is_none() {
                self.clipboard = Some(
                    arboard::Clipboard::new().map_err(|error| map_error("initialize", error))?,
                );
            }
            let clipboard = self
                .clipboard
                .as_mut()
                .ok_or_else(|| ClipboardError::Platform {
                    message: "clipboard initialization produced no handle".into(),
                })?;
            match operation {
                ClipboardOperation::Read => clipboard
                    .get_text()
                    .map(Some)
                    .map_err(|error| map_error("read text", error)),
                ClipboardOperation::Write(text) => clipboard
                    .set_text(text)
                    .map(|()| None)
                    .map_err(|error| map_error("write text", error)),
            }
        })
    }
}

fn execute_with(
    request: &ClipboardRequest,
    focused: bool,
    operation: impl FnOnce(&ClipboardOperation) -> Result<Option<String>, ClipboardError>,
) -> ClipboardResponse {
    ClipboardResponse {
        id: request.id,
        result: if focused {
            operation(&request.operation)
        } else {
            Err(ClipboardError::Unfocused)
        },
    }
}

fn map_error(operation: &str, error: arboard::Error) -> ClipboardError {
    match error {
        arboard::Error::ContentNotAvailable => ClipboardError::NoText,
        error => ClipboardError::Platform {
            message: format!("{operation}: {error}"),
        },
    }
}

#[cfg(test)]
mod test;
