mod errors;

use std::fmt;

use bevy_ecs::component::Component;

pub use errors::SceneIdError;

/// Stable, engine-owned identifier for one scene.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SceneId(String);

impl SceneId {
    /// Construct a scene identifier.
    ///
    /// # Errors
    ///
    /// Returns [`SceneIdError::EmptyIdentifier`] for empty or whitespace-only input.
    pub fn new(identifier: impl Into<String>) -> Result<Self, SceneIdError> {
        let identifier = identifier.into();
        if identifier.trim().is_empty() {
            return Err(SceneIdError::EmptyIdentifier);
        }
        Ok(Self(identifier))
    }

    /// Borrow the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SceneId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Component)]
pub(crate) struct SceneOwner(pub(crate) SceneId);

#[cfg(test)]
mod test;
