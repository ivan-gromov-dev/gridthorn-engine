//! Provisional game-owned digital action bindings, evaluated explicitly during Input.
mod binding;
mod mapping;
mod state;

pub use binding::{ActionBinding, BindingInput};
pub use mapping::{ActionBindings, ActionMapper};
pub use state::{ActionFrame, ActionState};

#[cfg(test)]
mod test;
