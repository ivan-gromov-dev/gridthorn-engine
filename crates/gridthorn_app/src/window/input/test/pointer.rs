use super::super::map_window_input;
use gridthorn_input::{InputEvent, Modifiers, ScrollPhase, WheelDelta};
use winit::event::{DeviceId, MouseScrollDelta, TouchPhase, WindowEvent};

#[test]
fn maps_native_wheel_units_and_every_phase() {
    for (native, engine) in [
        (TouchPhase::Started, ScrollPhase::Started),
        (TouchPhase::Moved, ScrollPhase::Moved),
        (TouchPhase::Ended, ScrollPhase::Ended),
        (TouchPhase::Cancelled, ScrollPhase::Cancelled),
    ] {
        assert_eq!(
            map_window_input(&WindowEvent::MouseWheel {
                device_id: DeviceId::dummy(),
                delta: MouseScrollDelta::LineDelta(-1.0, 2.0),
                phase: native
            }),
            Some(InputEvent::MouseWheel {
                delta: WheelDelta::Lines { x: -1.0, y: 2.0 },
                phase: engine
            })
        );
    }
    assert_eq!(
        map_window_input(&WindowEvent::MouseWheel {
            device_id: DeviceId::dummy(),
            delta: MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition::new(0.25, -3.5)),
            phase: TouchPhase::Moved
        }),
        Some(InputEvent::MouseWheel {
            delta: WheelDelta::Pixels { x: 0.25, y: -3.5 },
            phase: ScrollPhase::Moved
        })
    );
}

#[test]
fn maps_effective_modifiers_and_focus_changes() {
    let all = winit::keyboard::ModifiersState::SHIFT
        | winit::keyboard::ModifiersState::CONTROL
        | winit::keyboard::ModifiersState::ALT
        | winit::keyboard::ModifiersState::SUPER;
    assert_eq!(
        map_window_input(&WindowEvent::ModifiersChanged(all.into())),
        Some(InputEvent::ModifiersChanged(Modifiers {
            shift: true,
            control: true,
            alt: true,
            super_key: true
        }))
    );
    assert_eq!(
        map_window_input(&WindowEvent::Focused(false)),
        Some(InputEvent::FocusLost)
    );
    assert_eq!(
        map_window_input(&WindowEvent::Focused(true)),
        Some(InputEvent::FocusGained)
    );
}
