use super::*;
use gridthorn_input::controller::*;
fn button(id: u64, button: ControllerButton, state: ButtonState) -> InputEvent {
    InputEvent::Controller(ControllerEvent::Button {
        id: ControllerId(id),
        button,
        state,
    })
}
fn axis(value: f32) -> InputEvent {
    InputEvent::Controller(ControllerEvent::Axis {
        id: ControllerId(1),
        axis: ControllerAxis::LeftStickY,
        value,
    })
}
#[test]
fn controller_opt_in_taps_duplicate_suppression_and_unmapped_passthrough() {
    let mut tree = tree(vec![
        UiControl::Button("first".into()),
        UiControl::Toggle {
            label: "second".into(),
            checked: false,
        },
    ]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(0);
    let next = button(1, ControllerButton::RightShoulder, ButtonState::Pressed);
    assert_eq!(
        router
            .route_events(&mut tree, &layout, std::slice::from_ref(&next))
            .unwrap()
            .world_events,
        std::slice::from_ref(&next)
    );
    router.set_controller_navigation(true);
    let events = [
        next.clone(),
        next,
        button(1, ControllerButton::RightShoulder, ButtonState::Released),
        button(1, ControllerButton::South, ButtonState::Pressed),
        button(1, ControllerButton::South, ButtonState::Released),
        button(1, ControllerButton::Start, ButtonState::Pressed),
    ];
    let route = router.route_events(&mut tree, &layout, &events).unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(1)));
    assert_eq!(route.effects, [(UiNodeId(1), UiEffect::Activated)]);
    assert_eq!(route.consumed, [0, 1, 2, 3, 4]);
    assert_eq!(route.world_events, [events[5].clone()]);
    assert!(route.controller_blocked);
}
#[test]
fn stick_hysteresis_rearms_and_connection_loss_clears_ownership() {
    let mut tree = tree(vec![
        UiControl::Button("first".into()),
        UiControl::Button("second".into()),
        UiControl::Button("third".into()),
    ]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(0);
    router.set_controller_navigation(true);
    router
        .route_events(&mut tree, &layout, &[axis(-0.7)])
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(1)));
    router
        .route_events(&mut tree, &layout, &[axis(-0.5), axis(-0.8)])
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(1)));
    router
        .route_events(&mut tree, &layout, &[axis(0.0), axis(-0.7)])
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(2)));
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                InputEvent::Controller(ControllerEvent::Disconnected(ControllerId(1))),
                axis(-0.7),
            ],
        )
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(3)));
    router
        .route_events(
            &mut tree,
            &layout,
            &[button(1, ControllerButton::East, ButtonState::Pressed)],
        )
        .unwrap();
    assert_eq!(router.focused(), None);
}
