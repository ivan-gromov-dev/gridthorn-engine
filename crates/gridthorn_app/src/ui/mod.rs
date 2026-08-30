mod errors;

pub use errors::UiButtonError;

use gridthorn_input::{InputState, MouseButton};

/// Frame-scoped interaction state for one runtime UI button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiButtonInteraction {
    hovered: bool,
    pressed: bool,
    activated: bool,
}

impl UiButtonInteraction {
    /// Whether the cursor is currently inside the button bounds.
    #[must_use]
    pub const fn hovered(self) -> bool {
        self.hovered
    }

    /// Whether the primary mouse button is held over this button.
    #[must_use]
    pub const fn pressed(self) -> bool {
        self.pressed
    }

    /// Whether an inside press completed with an inside release this frame.
    #[must_use]
    pub const fn activated(self) -> bool {
        self.activated
    }
}

/// Persistent mouse-interaction state for one screen-space button.
///
/// Call [`Self::update`] once during `Input` with the frame's [`InputState`].
/// A press arms only inside the bounds, and activation occurs once when that
/// press is released inside. Dragging or focus loss outside cancels activation.
pub struct UiButton {
    position: [f32; 2],
    size: [f32; 2],
    armed: bool,
}

impl UiButton {
    /// Construct a button in top-left-origin physical screen pixels.
    ///
    /// # Errors
    ///
    /// Returns [`UiButtonError::InvalidPosition`] for non-finite coordinates
    /// and [`UiButtonError::InvalidSize`] for invalid dimensions.
    pub fn new(position: [f32; 2], size: [f32; 2]) -> Result<Self, UiButtonError> {
        if !position.into_iter().all(f32::is_finite) {
            return Err(UiButtonError::InvalidPosition);
        }
        if !size
            .into_iter()
            .all(|dimension| dimension.is_finite() && dimension > 0.0)
        {
            return Err(UiButtonError::InvalidSize);
        }
        Ok(Self {
            position,
            size,
            armed: false,
        })
    }

    /// Top-left button position in physical screen pixels.
    #[must_use]
    pub const fn position(&self) -> [f32; 2] {
        self.position
    }

    /// Button width and height in physical screen pixels.
    #[must_use]
    pub const fn size(&self) -> [f32; 2] {
        self.size
    }

    /// Consume one frame of engine-owned input and return its interaction state.
    pub fn update(&mut self, input: &InputState) -> UiButtonInteraction {
        let hovered = input
            .cursor_position()
            .is_some_and(|cursor| self.contains(cursor.x, cursor.y));
        if input.mouse_button_just_pressed(MouseButton::Left) {
            self.armed = hovered;
        }
        let activated =
            input.mouse_button_just_released(MouseButton::Left) && self.armed && hovered;
        if input.mouse_button_just_released(MouseButton::Left) {
            self.armed = false;
        }
        UiButtonInteraction {
            hovered,
            pressed: hovered && self.armed && input.mouse_button_down(MouseButton::Left),
            activated,
        }
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        let left = f64::from(self.position[0]);
        let top = f64::from(self.position[1]);
        x >= left
            && x < left + f64::from(self.size[0])
            && y >= top
            && y < top + f64::from(self.size[1])
    }
}

#[cfg(test)]
mod test;
