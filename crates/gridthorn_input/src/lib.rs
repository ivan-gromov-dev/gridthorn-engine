//! Provisional engine-owned keyboard and mouse input contracts.

mod event;
mod state;

pub use event::{ButtonState, CursorPosition, InputEvent, KeyCode, MouseButton};
pub use state::{InputBuffer, InputState};
