use std::collections::{BTreeMap, BTreeSet};

use super::{ActionBinding, ActionFrame};
use crate::{
    InputBuffer, InputEvent, InputState, controller::ControllerEvent, devices::DeviceEvent,
};

/// Caller-owned, replaceable digital bindings. No game action names or persistence policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionBindings<A> {
    bindings: BTreeMap<A, Vec<ActionBinding>>,
}

impl<A> Default for ActionBindings<A> {
    fn default() -> Self {
        Self {
            bindings: BTreeMap::new(),
        }
    }
}

impl<A: Ord> ActionBindings<A> {
    /// Replace an action's alternatives. Empty alternatives disable the action.
    pub fn set(&mut self, action: A, alternatives: Vec<ActionBinding>) {
        self.bindings.insert(action, alternatives);
    }

    /// Remove an action. A mapper reports release on its next evaluation if held.
    pub fn remove(&mut self, action: &A) {
        self.bindings.remove(action);
    }

    /// Inspect the configured alternatives without changing held state.
    #[must_use]
    pub fn get(&self, action: &A) -> &[ActionBinding] {
        self.bindings.get(action).map_or(&[], Vec::as_slice)
    }
}

/// Per-consumer action history. Evaluate once per host frame in Input, before fixed ticks.
/// Filter consumed UI events before evaluation and suppress owned continuous inputs.
pub struct ActionMapper<A> {
    input: InputBuffer,
    active: BTreeSet<A>,
}

impl<A> Default for ActionMapper<A> {
    fn default() -> Self {
        Self {
            input: InputBuffer::new(),
            active: BTreeSet::new(),
        }
    }
}

impl<A: Ord + Clone> ActionMapper<A> {
    /// Create an independent mapper with no previous held actions.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluate ordered transitions, then reconcile held state with the final snapshot.
    /// Any held alternative activates the action; repeats never add another press edge.
    /// A config change applies against held input on this call. Focus/disconnect
    /// cancellation is distinct from normal release. No events are consumed or serialized.
    /// Calls that skip snapshots can recover held state, but cannot recover skipped taps.
    #[must_use]
    pub fn evaluate(&mut self, bindings: &ActionBindings<A>, input: &InputState) -> ActionFrame<A> {
        let mut frame = ActionFrame {
            actions: BTreeMap::new(),
        };
        Self::transition(
            bindings,
            self.input.state(),
            &mut self.active,
            &mut frame,
            false,
        );
        for event in input.events() {
            if !matches!(
                event,
                InputEvent::Capabilities(_)
                    | InputEvent::RawInputChanged { .. }
                    | InputEvent::Device(_)
                    | InputEvent::Controller(_)
                    | InputEvent::Key(_)
                    | InputEvent::Keyboard { .. }
                    | InputEvent::MouseButton { .. }
                    | InputEvent::ModifiersChanged(_)
                    | InputEvent::FocusLost
                    | InputEvent::FocusGained
            ) {
                continue;
            }
            self.input.apply(event);
            let cancelled = matches!(
                event,
                InputEvent::FocusLost
                    | InputEvent::RawInputChanged { enabled: false, .. }
                    | InputEvent::Device(DeviceEvent::Disconnected(_))
                    | InputEvent::Controller(ControllerEvent::Disconnected(_))
            );
            Self::transition(
                bindings,
                self.input.state(),
                &mut self.active,
                &mut frame,
                cancelled,
            );
        }
        Self::transition(bindings, input, &mut self.active, &mut frame, false);
        self.input.synchronize(input);
        frame
    }

    fn transition(
        bindings: &ActionBindings<A>,
        input: &InputState,
        active: &mut BTreeSet<A>,
        frame: &mut ActionFrame<A>,
        cancelled: bool,
    ) {
        let next: BTreeSet<A> = bindings
            .bindings
            .iter()
            .filter(|(_, alternatives)| alternatives.iter().any(|binding| binding.down(input)))
            .map(|(action, _)| action.clone())
            .collect();
        for action in active.difference(&next) {
            let state = frame.actions.entry(action.clone()).or_default();
            state.down = false;
            if cancelled {
                state.cancelled = true;
            } else {
                state.just_released = true;
            }
        }
        for action in bindings.bindings.keys() {
            let state = frame.actions.entry(action.clone()).or_default();
            state.down = next.contains(action);
            state.just_pressed |= state.down && !active.contains(action);
        }
        *active = next;
    }
}
