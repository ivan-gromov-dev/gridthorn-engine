/// Physical-pixel rectangle used to position native IME candidate windows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImeCursorArea {
    /// Left edge in physical window pixels.
    pub x: f64,
    /// Top edge in physical window pixels.
    pub y: f64,
    /// Nonnegative physical width.
    pub width: f64,
    /// Nonnegative physical height.
    pub height: f64,
}

impl ImeCursorArea {
    /// Validate a candidate-window anchor.
    ///
    /// # Errors
    /// Returns `TextInputError::InvalidCursorArea` for nonfinite coordinates or negative extents.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Result<Self, crate::TextInputError> {
        let area = Self {
            x,
            y,
            width,
            height,
        };
        area.validate()?;
        Ok(area)
    }

    /// Validate externally constructed rectangles.
    ///
    /// # Errors
    /// Returns an error for nonfinite coordinates or negative extents.
    pub fn validate(self) -> Result<(), crate::TextInputError> {
        if [self.x, self.y, self.width, self.height]
            .iter()
            .all(|value| value.is_finite())
            && self.width >= 0.0
            && self.height >= 0.0
        {
            Ok(())
        } else {
            Err(crate::TextInputError::InvalidCursorArea)
        }
    }
}

/// Engine-owned text stream, independent of physical shortcut events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TextInputEvent {
    /// Insert this UTF-8 text once; no normalization or grapheme splitting is performed.
    Commit(String),
    /// Replace the current preedit. Cursor endpoints are UTF-8 byte offsets.
    Composition {
        /// Uncommitted Unicode text.
        text: String,
        /// Optional caret/selection endpoints in UTF-8 bytes.
        cursor: Option<(usize, usize)>,
    },
    /// Discard preedit without inserting it.
    CompositionCancelled,
    /// The operating system enabled composition.
    ImeEnabled,
    /// The operating system disabled composition.
    ImeDisabled,
}

/// One-shot text-session operation processed by a platform adapter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextInputRequest {
    /// Open or update the session and candidate anchor.
    Start(ImeCursorArea),
    /// Close the session and cancel composition.
    Stop,
}

/// Runtime resource for text-session requests, applied after the current frame.
#[derive(Debug, Default)]
pub struct TextInput {
    pending: Option<TextInputRequest>,
}

impl TextInput {
    /// Open or update a text session at a physical-pixel candidate anchor.
    ///
    /// # Errors
    /// Invalid rectangles leave the pending request unchanged.
    pub fn start(&mut self, area: ImeCursorArea) -> Result<(), crate::TextInputError> {
        area.validate()?;
        self.pending = Some(TextInputRequest::Start(area));
        Ok(())
    }

    /// Close a session and cancel outstanding preedit. Last request wins.
    pub fn stop(&mut self) {
        self.pending = Some(TextInputRequest::Stop);
    }

    /// Take one pending request for a platform adapter.
    pub fn take_request(&mut self) -> Option<TextInputRequest> {
        self.pending.take()
    }
}

#[cfg(test)]
mod test;
