use super::*;
use gridthorn_input::{
    ClipboardOperation, ClipboardResponse, KeyLocation, KeyboardEvent, LogicalKey, Modifiers,
    PhysicalKey,
};

fn shortcut(character: &str) -> InputEvent {
    InputEvent::Key(KeyboardEvent {
        physical_key: PhysicalKey::Code(KeyCode::KeyV),
        logical_key: LogicalKey::Character(character.into()),
        location: KeyLocation::Standard,
        state: ButtonState::Pressed,
        repeat: false,
        synthetic: false,
    })
}
fn response(
    id: u64,
    result: Result<Option<String>, gridthorn_input::ClipboardError>,
) -> InputEvent {
    InputEvent::Clipboard(ClipboardResponse { id, result })
}
fn focused() -> (UiTree, UiLayout, UiRouter) {
    let mut tree = field("Привет");
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                InputEvent::ModifiersChanged(Modifiers {
                    control: true,
                    ..Default::default()
                }),
                shortcut("a"),
            ],
        )
        .unwrap();
    (tree, layout, router)
}

#[test]
fn copy_and_paste_correlate_only_owned_responses() {
    let (mut tree, layout, mut router) = focused();
    let result = router
        .route_events(&mut tree, &layout, &[shortcut("c"), shortcut("v")])
        .unwrap();
    assert!(result.platform.iter().any(|request| matches!(request, UiPlatformRequest::Clipboard(request) if request.id == 100 && request.operation == ClipboardOperation::Write("Привет".into()))));
    let result = router
        .route_events(
            &mut tree,
            &layout,
            &[
                response(999, Ok(Some("wrong".into()))),
                response(101, Ok(Some("日本語".into()))),
            ],
        )
        .unwrap();
    assert_eq!(value(&tree), "日本語");
    assert_eq!(result.world_events.len(), 1);
    assert_eq!(result.clipboard.len(), 1);
}

#[test]
fn cut_waits_for_success_and_clipboard_failure_preserves_text() {
    let (mut tree, layout, mut router) = focused();
    router
        .route_events(&mut tree, &layout, &[shortcut("x")])
        .unwrap();
    assert_eq!(value(&tree), "Привет");
    let result = router
        .route_events(
            &mut tree,
            &layout,
            &[response(
                100,
                Err(gridthorn_input::ClipboardError::Unfocused),
            )],
        )
        .unwrap();
    assert_eq!(value(&tree), "Привет");
    assert!(result.clipboard[0].result.is_err());
    router
        .route_events(
            &mut tree,
            &layout,
            &[shortcut("x"), response(101, Ok(None))],
        )
        .unwrap();
    assert_eq!(value(&tree), "");
}

#[test]
fn delayed_paste_is_discarded_after_selection_value_or_focus_changes() {
    let (mut tree, layout, mut router) = focused();
    router
        .route_events(&mut tree, &layout, &[shortcut("v")])
        .unwrap();
    router
        .select(
            &tree,
            UiSelection {
                anchor: 0,
                caret: 0,
            },
        )
        .unwrap();
    router
        .route_events(
            &mut tree,
            &layout,
            &[response(100, Ok(Some("late".into())))],
        )
        .unwrap();
    assert_eq!(value(&tree), "Привет");
    router
        .route_events(&mut tree, &layout, &[shortcut("v")])
        .unwrap();
    tree.command(UiNodeId(1), UiCommand::SetText("external".into()))
        .unwrap();
    router
        .route_events(
            &mut tree,
            &layout,
            &[response(101, Ok(Some("late".into())))],
        )
        .unwrap();
    assert_eq!(value(&tree), "external");
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                shortcut("v"),
                InputEvent::FocusLost,
                response(102, Ok(Some("late".into()))),
            ],
        )
        .unwrap();
    assert_eq!(value(&tree), "external");
}
