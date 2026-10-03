use super::UiCompositionError;
use unicode_segmentation::UnicodeSegmentation;

/// UTF-8 byte endpoints at extended grapheme boundaries; anchor may follow caret.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiSelection {
    /// Fixed end during Shift navigation or pointer dragging.
    pub anchor: usize,
    /// Active insertion end.
    pub caret: usize,
}

impl UiSelection {
    /// Ordered selected byte range.
    #[must_use]
    pub fn range(self) -> std::ops::Range<usize> {
        self.anchor.min(self.caret)..self.anchor.max(self.caret)
    }

    /// Validate both endpoints against extended grapheme boundaries.
    ///
    /// # Errors
    /// Rejects out-of-range, scalar-interior and grapheme-interior endpoints.
    pub fn validate(self, value: &str) -> Result<(), UiCompositionError> {
        let stops = boundaries(value);
        if stops.contains(&self.anchor) && stops.contains(&self.caret) {
            Ok(())
        } else {
            Err(UiCompositionError::InvalidMetrics("text selection"))
        }
    }
}

pub(super) fn boundaries(value: &str) -> Vec<usize> {
    value
        .grapheme_indices(true)
        .map(|(index, _)| index)
        .chain(std::iter::once(value.len()))
        .collect()
}

#[derive(Clone, Debug, Default)]
pub(super) struct Editor {
    pub value: String,
    pub selection: UiSelection,
    pub preedit: String,
    pub preedit_cursor: Option<(usize, usize)>,
}

impl Editor {
    pub fn cancel_composition(&mut self) {
        self.preedit.clear();
        self.preedit_cursor = None;
    }
    pub fn sync(&mut self, value: &str) {
        if self.value != value {
            value.clone_into(&mut self.value);
            self.selection = UiSelection {
                anchor: value.len(),
                caret: value.len(),
            };
            self.cancel_composition();
        }
    }

    pub fn move_to(&mut self, byte: usize, extend: bool) {
        self.selection.caret = byte;
        if !extend {
            self.selection.anchor = byte;
        }
    }

    pub fn step(&mut self, forward: bool, extend: bool) {
        let range = self.selection.range();
        if !extend && !range.is_empty() {
            self.move_to(if forward { range.end } else { range.start }, false);
            return;
        }
        let stops = boundaries(&self.value);
        let index = stops
            .iter()
            .position(|byte| *byte == self.selection.caret)
            .unwrap_or(0);
        let next = if forward {
            (index + 1).min(stops.len() - 1)
        } else {
            index.saturating_sub(1)
        };
        self.move_to(stops[next], extend);
    }

    pub fn replace(&mut self, text: &str) -> Result<String, UiCompositionError> {
        let range = self.selection.range();
        if self.value.len() - range.len() + text.len() > 65536 {
            return Err(UiCompositionError::InvalidMetrics("text length"));
        }
        let mut value = self.value.clone();
        value.replace_range(range.clone(), text);
        let insertion_end = range.start + text.len();
        let caret = boundaries(&value)
            .into_iter()
            .find(|byte| *byte >= insertion_end)
            .unwrap_or(value.len());
        self.value.clone_from(&value);
        self.move_to(caret, false);
        self.cancel_composition();
        Ok(value)
    }

    pub fn delete(&mut self, forward: bool) -> Result<String, UiCompositionError> {
        if self.selection.range().is_empty() {
            self.step(forward, true);
        }
        self.replace("")
    }
}
