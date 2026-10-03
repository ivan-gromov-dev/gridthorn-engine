use super::super::*;
use gridthorn_input::{ButtonState, CursorPosition, InputEvent, KeyCode, MouseButton};
mod clipboard;
mod editing;
mod navigation;
mod pointer;

fn tree(controls: Vec<UiControl>) -> UiTree {
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.style.flow = UiFlow::Column;
    for (index, control) in controls.into_iter().enumerate() {
        let mut node = UiNode::new(UiNodeId(u64::try_from(index).unwrap() + 1), control);
        node.style.size = [UiLength::Pixels(100.0), UiLength::Pixels(30.0)];
        root.children.push(node);
    }
    UiTree::new(root, UiTheme::default()).unwrap()
}

fn key(key: KeyCode, state: ButtonState) -> InputEvent {
    InputEvent::Keyboard { key, state }
}
fn press(key_code: KeyCode) -> InputEvent {
    key(key_code, ButtonState::Pressed)
}
fn pointer(x: f64, y: f64) -> InputEvent {
    InputEvent::CursorMoved(CursorPosition { x, y })
}
fn mouse(state: ButtonState) -> InputEvent {
    InputEvent::MouseButton {
        button: MouseButton::Left,
        state,
    }
}
fn value(tree: &UiTree) -> &str {
    let UiControl::TextField { value, .. } = &tree.node(UiNodeId(1)).unwrap().control else {
        panic!("field");
    };
    value
}
fn field(value: &str) -> UiTree {
    tree(vec![UiControl::TextField {
        value: value.into(),
        placeholder: "type".into(),
    }])
}
