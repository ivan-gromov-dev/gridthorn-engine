use crate::actions::{ActionBinding, ActionBindings, ActionMapper, BindingInput};
use crate::input_devices::{DeviceDiscovery, InputCapabilities, InputSource};
use crate::{
    ApplicationRuntime, ButtonState, InputBuffer, InputEvent, InputState, KeyCode, PhysicalKey,
    ScheduleBuilder, ScheduleStage,
};

#[derive(Default)]
struct Actions {
    mapper: ActionMapper<&'static str>,
    presses: usize,
}

#[test]
fn public_actions_run_once_in_input_before_multiple_fixed_ticks() {
    let mut bindings = ActionBindings::default();
    bindings.set(
        "jump",
        vec![ActionBinding::new(BindingInput::Key(PhysicalKey::Code(
            KeyCode::Space,
        )))],
    );
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Input, |world| {
        let input = world
            .read_resource(Clone::clone)
            .unwrap_or_else(InputState::default);
        assert!(InputSource::Keyboard.validate(&input).is_ok());
        let bindings = world
            .read_resource(Clone::clone)
            .unwrap_or_else(ActionBindings::default);
        world.update_resource(|actions: &mut Actions| {
            actions.presses += usize::from(
                actions
                    .mapper
                    .evaluate(&bindings, &input)
                    .get(&"jump")
                    .just_pressed,
            );
        });
    });
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        assert_eq!(
            world.read_resource(|actions: &Actions| actions.presses),
            Some(1)
        );
    });
    let mut app = ApplicationRuntime::new(schedules.build());
    app.world().insert_resource(bindings);
    app.world().insert_resource(Actions::default());
    let mut input = InputBuffer::new();
    input.push(InputEvent::Capabilities(InputCapabilities {
        aggregate_keyboard: true,
        aggregate_mouse: true,
        raw_devices: DeviceDiscovery::Unsupported,
    }));
    input.push(InputEvent::Keyboard {
        key: KeyCode::Space,
        state: ButtonState::Pressed,
    });
    input.push(InputEvent::Keyboard {
        key: KeyCode::Space,
        state: ButtonState::Released,
    });
    app.world().insert_resource(input.snapshot());
    app.run_frame(2).unwrap();
    app.world().insert_resource(input.snapshot());
    app.run_frame(2).unwrap();
}
