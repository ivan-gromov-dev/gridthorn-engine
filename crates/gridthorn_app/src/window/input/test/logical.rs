use super::super::*;
use gridthorn_input::{LogicalKey, NamedKey, NativeKey, PhysicalKey as EnginePhysical};

#[test]
fn maps_layout_characters_dead_named_and_native_keys() {
    assert_eq!(
        map_logical_key(&winit::keyboard::Key::Character("й".into())),
        LogicalKey::Character("й".into())
    );
    assert_eq!(
        map_logical_key(&winit::keyboard::Key::Dead(Some('^'))),
        LogicalKey::Dead(Some('^'))
    );
    assert_eq!(
        map_logical_key(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::F35)),
        LogicalKey::Named(NamedKey::F35)
    );
    assert_eq!(
        map_logical_key(&winit::keyboard::Key::Unidentified(
            winit::keyboard::NativeKey::Windows(123)
        )),
        LogicalKey::Unidentified(NativeKey::Windows(123))
    );
    assert_eq!(
        map_physical_key(PhysicalKey::Unidentified(
            winit::keyboard::NativeKeyCode::Xkb(222)
        )),
        EnginePhysical::Unidentified(NativeKey::Xkb(222))
    );
}
