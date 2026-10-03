use super::super::RuntimeWindowLifecycle;
use crate::{ApplicationRuntime, WindowControl, WindowLifecycle};
use gridthorn_input::{
    Clipboard, ClipboardOperation, ImeCursorArea, InputEvent, InputState, TextInput,
    TextInputEvent, TextInputRequest,
};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

#[test]
fn startup_requests_drain_once_and_feedback_reaches_input() {
    let area = ImeCursorArea::new(12.0, 24.0, 1.0, 18.0).unwrap();
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        assert!(world.update_resource(|text: &mut TextInput| text.start(area).unwrap()));
        assert!(world.update_resource(|clipboard: &mut Clipboard| {
            clipboard.write(1, "Привет");
            clipboard.read(2);
        }));
    });
    schedules.add_system(ScheduleStage::Input, |world| {
        let input = world
            .read_resource(|input: &InputState| input.clone())
            .unwrap();
        world.insert_resource(input.text_input_active());
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    lifecycle.started(&mut WindowControl::default()).unwrap();
    let mut control = WindowControl::default();
    lifecycle.idle(&mut control).unwrap();
    assert_eq!(control.text_input, Some(TextInputRequest::Start(area)));
    assert_eq!(control.clipboard.len(), 2);
    assert_eq!(
        control.clipboard[0].operation,
        ClipboardOperation::Write("Привет".into())
    );
    lifecycle
        .input(InputEvent::TextInputChanged {
            active: true,
            error: None,
        })
        .unwrap();
    lifecycle
        .input(InputEvent::Text(TextInputEvent::Composition {
            text: "日本".into(),
            cursor: Some((3, 6)),
        }))
        .unwrap();
    let mut next = WindowControl::default();
    lifecycle.idle(&mut next).unwrap();
    assert_eq!(next.text_input, None);
    assert_eq!(next.clipboard, []);
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|active: &bool| *active),
        Some(true)
    );
    lifecycle.suspended();
    lifecycle.resumed();
    lifecycle.idle(&mut WindowControl::default()).unwrap();
    lifecycle
        .runtime
        .world()
        .read_resource(|input: &InputState| {
            assert!(!input.text_input_active());
            assert!(input.composition().is_none());
            assert_eq!(
                input.events(),
                &[
                    InputEvent::FocusLost,
                    InputEvent::Text(TextInputEvent::CompositionCancelled)
                ]
            );
        })
        .unwrap();
}
