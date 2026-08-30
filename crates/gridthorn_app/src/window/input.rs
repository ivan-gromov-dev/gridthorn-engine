use gridthorn_input::{
    ButtonState, CursorPosition, InputEvent, KeyCode as EngineKeyCode,
    MouseButton as EngineMouseButton,
};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub(super) fn map_window_input(event: &WindowEvent) -> Option<InputEvent> {
    match event {
        WindowEvent::KeyboardInput { event, .. } => {
            let PhysicalKey::Code(key) = event.physical_key else {
                return None;
            };
            Some(InputEvent::Keyboard {
                key: map_key_code(key)?,
                state: map_button_state(event.state),
            })
        }
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

fn map_key_code(key: KeyCode) -> Option<EngineKeyCode> {
    match key {
        KeyCode::KeyW => Some(EngineKeyCode::KeyW),
        KeyCode::KeyA => Some(EngineKeyCode::KeyA),
        KeyCode::KeyS => Some(EngineKeyCode::KeyS),
        KeyCode::KeyD => Some(EngineKeyCode::KeyD),
        KeyCode::ArrowUp => Some(EngineKeyCode::ArrowUp),
        KeyCode::ArrowDown => Some(EngineKeyCode::ArrowDown),
        KeyCode::ArrowLeft => Some(EngineKeyCode::ArrowLeft),
        KeyCode::ArrowRight => Some(EngineKeyCode::ArrowRight),
        KeyCode::Space => Some(EngineKeyCode::Space),
        KeyCode::Enter => Some(EngineKeyCode::Enter),
        KeyCode::Escape => Some(EngineKeyCode::Escape),
        _ => None,
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
