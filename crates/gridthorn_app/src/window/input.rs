use gridthorn_input::{
    ButtonState, CursorPosition, InputEvent, KeyCode as EngineKeyCode,
    MouseButton as EngineMouseButton,
};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub(super) fn map_window_input(event: &WindowEvent) -> Option<InputEvent> {
    match event {
        WindowEvent::KeyboardInput {
            event,
            is_synthetic,
            ..
        } => Some(InputEvent::Key(gridthorn_input::KeyboardEvent {
            physical_key: map_physical_key(event.physical_key),
            logical_key: map_logical_key(&event.logical_key),
            location: match event.location {
                winit::keyboard::KeyLocation::Standard => gridthorn_input::KeyLocation::Standard,
                winit::keyboard::KeyLocation::Left => gridthorn_input::KeyLocation::Left,
                winit::keyboard::KeyLocation::Right => gridthorn_input::KeyLocation::Right,
                winit::keyboard::KeyLocation::Numpad => gridthorn_input::KeyLocation::Numpad,
            },
            state: map_button_state(event.state),
            repeat: event.repeat,
            synthetic: *is_synthetic,
        })),
        WindowEvent::ModifiersChanged(modifiers) => {
            let state = modifiers.state();
            Some(InputEvent::ModifiersChanged(gridthorn_input::Modifiers {
                shift: state.shift_key(),
                control: state.control_key(),
                alt: state.alt_key(),
                super_key: state.super_key(),
            }))
        }
        WindowEvent::MouseWheel { delta, phase, .. } => Some(InputEvent::MouseWheel {
            delta: match delta {
                winit::event::MouseScrollDelta::LineDelta(x, y) => {
                    gridthorn_input::WheelDelta::Lines { x: *x, y: *y }
                }
                winit::event::MouseScrollDelta::PixelDelta(p) => {
                    gridthorn_input::WheelDelta::Pixels { x: p.x, y: p.y }
                }
            },
            phase: match phase {
                winit::event::TouchPhase::Started => gridthorn_input::ScrollPhase::Started,
                winit::event::TouchPhase::Moved => gridthorn_input::ScrollPhase::Moved,
                winit::event::TouchPhase::Ended => gridthorn_input::ScrollPhase::Ended,
                winit::event::TouchPhase::Cancelled => gridthorn_input::ScrollPhase::Cancelled,
            },
        }),
        WindowEvent::Focused(true) => Some(InputEvent::FocusGained),
        WindowEvent::MouseInput { state, button, .. } => Some(InputEvent::MouseButton {
            button: map_mouse_button(*button),
            state: map_button_state(*state),
        }),
        WindowEvent::CursorMoved { position, .. } => {
            Some(InputEvent::CursorMoved(CursorPosition {
                x: position.x,
                y: position.y,
            }))
        }
        WindowEvent::CursorLeft { .. } => Some(InputEvent::CursorLeft),
        WindowEvent::Focused(false) => Some(InputEvent::FocusLost),
        _ => None,
    }
}

use super::key_mapping::{map_keycode, map_namedkey};

fn map_key_code(key: KeyCode) -> EngineKeyCode {
    map_keycode(key)
}

fn map_physical_key(key: PhysicalKey) -> gridthorn_input::PhysicalKey {
    use gridthorn_input::{NativeKey, PhysicalKey as P};
    match key {
        PhysicalKey::Code(code) => P::Code(map_key_code(code)),
        PhysicalKey::Unidentified(native) => P::Unidentified(match native {
            winit::keyboard::NativeKeyCode::Unidentified => NativeKey::Unidentified,
            winit::keyboard::NativeKeyCode::Android(code) => NativeKey::Android(code),
            winit::keyboard::NativeKeyCode::MacOS(code) => NativeKey::MacOS(code),
            winit::keyboard::NativeKeyCode::Windows(code) => NativeKey::Windows(code),
            winit::keyboard::NativeKeyCode::Xkb(code) => NativeKey::Xkb(code),
        }),
    }
}

fn map_logical_key(key: &winit::keyboard::Key) -> gridthorn_input::LogicalKey {
    use gridthorn_input::{LogicalKey as L, NativeKey};
    match key {
        winit::keyboard::Key::Named(key) => L::Named(map_namedkey(*key)),
        winit::keyboard::Key::Character(text) => L::Character(text.to_string()),
        winit::keyboard::Key::Dead(accent) => L::Dead(*accent),
        winit::keyboard::Key::Unidentified(native) => L::Unidentified(match native {
            winit::keyboard::NativeKey::Unidentified => NativeKey::Unidentified,
            winit::keyboard::NativeKey::Android(code) => NativeKey::Android(*code),
            winit::keyboard::NativeKey::MacOS(code) => NativeKey::MacOS(*code),
            winit::keyboard::NativeKey::Windows(code) => NativeKey::Windows(*code),
            winit::keyboard::NativeKey::Xkb(code) => NativeKey::Xkb(*code),
            winit::keyboard::NativeKey::Web(text) => NativeKey::Web(text.to_string()),
        }),
    }
}
fn map_mouse_button(button: MouseButton) -> EngineMouseButton {
    match button {
        MouseButton::Left => EngineMouseButton::Left,
        MouseButton::Right => EngineMouseButton::Right,
        MouseButton::Middle => EngineMouseButton::Middle,
        MouseButton::Back => EngineMouseButton::Back,
        MouseButton::Forward => EngineMouseButton::Forward,
        MouseButton::Other(number) => EngineMouseButton::Other(number),
    }
}

fn map_button_state(state: ElementState) -> ButtonState {
    match state {
        ElementState::Pressed => ButtonState::Pressed,
        ElementState::Released => ButtonState::Released,
    }
}

#[cfg(test)]
mod test;
