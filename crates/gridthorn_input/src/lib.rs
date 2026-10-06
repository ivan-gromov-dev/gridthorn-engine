//! Provisional engine-owned desktop and controller input contracts.

pub mod controller;
mod event;
mod key_code;
mod keyboard;
mod named_key;
mod pointer;
mod state;

pub use event::{ButtonState, CursorPosition, InputEvent, MouseButton};
pub use key_code::KeyCode;
pub use keyboard::{KeyLocation, KeyboardEvent, LogicalKey, Modifiers, NativeKey, PhysicalKey};
pub use named_key::NamedKey;
pub use pointer::{
    PointerCapture, PointerCaptureMode, PointerCaptureStatus, ScrollPhase, WheelDelta,
};
pub use state::{InputBuffer, InputState};
mod errors;
pub use errors::PointerCaptureError;
mod clipboard;
mod text;
pub use clipboard::{Clipboard, ClipboardOperation, ClipboardRequest, ClipboardResponse};
pub use errors::{ClipboardError, TextInputError};
pub use text::{ImeCursorArea, TextInput, TextInputEvent, TextInputRequest};
