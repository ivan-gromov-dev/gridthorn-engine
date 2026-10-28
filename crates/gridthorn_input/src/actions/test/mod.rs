mod desktop;
mod devices;

use super::{ActionBinding, ActionBindings, ActionMapper, BindingInput};
use crate::{ButtonState, InputBuffer, InputEvent, KeyCode, PhysicalKey};

fn key_binding(key: KeyCode) -> ActionBinding {
    ActionBinding::new(BindingInput::Key(PhysicalKey::Code(key)))
}

fn key(input: &mut InputBuffer, key: KeyCode, state: ButtonState) {
    input.push(InputEvent::Keyboard { key, state });
}

fn map() -> (
    InputBuffer,
    ActionBindings<&'static str>,
    ActionMapper<&'static str>,
) {
    let mut bindings = ActionBindings::default();
    bindings.set("jump", vec![key_binding(KeyCode::Space)]);
    (InputBuffer::new(), bindings, ActionMapper::default())
}
