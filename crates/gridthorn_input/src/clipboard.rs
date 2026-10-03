/// Plain-text clipboard operation. Rich formats and selection clipboards are outside this contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClipboardOperation {
    /// Read Unicode text.
    Read,
    /// Replace clipboard contents with Unicode text.
    Write(String),
}

/// Caller-correlated clipboard request, executed on the native event-loop thread.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClipboardRequest {
    /// Caller-owned correlation identity.
    pub id: u64,
    /// Requested operation.
    pub operation: ClipboardOperation,
}

/// Clipboard feedback. Reads return `Some(text)`; successful writes return `None`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClipboardResponse {
    /// Identity from the original request.
    pub id: u64,
    /// Unicode content or a contextual recoverable error.
    pub result: Result<Option<String>, crate::ClipboardError>,
}

/// Ordered one-shot clipboard requests; native runtimes install this before startup.
#[derive(Debug, Default)]
pub struct Clipboard {
    pending: Vec<ClipboardRequest>,
}

impl Clipboard {
    /// Request a read; choose an identity that distinguishes outstanding operations.
    pub fn read(&mut self, id: u64) {
        self.pending.push(ClipboardRequest {
            id,
            operation: ClipboardOperation::Read,
        });
    }

    /// Request a write; clipboard contents change only when the adapter executes it.
    pub fn write(&mut self, id: u64, text: impl Into<String>) {
        self.pending.push(ClipboardRequest {
            id,
            operation: ClipboardOperation::Write(text.into()),
        });
    }

    /// Drain pending requests in submission order for a platform adapter.
    pub fn take_requests(&mut self) -> Vec<ClipboardRequest> {
        std::mem::take(&mut self.pending)
    }
}

#[cfg(test)]
mod test;
