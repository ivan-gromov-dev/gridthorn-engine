use std::collections::BTreeMap;

use gridthorn_world::{Reflect, ReflectValue, ReflectionRegistry};

use super::{
    SceneError, SceneValueError,
    prepared::{ComponentCommit, ResourceCommit},
};

/// Explicit opt-in to authoritative scene persistence and domain validation.
///
/// Construction receives validated scalar values in `Reflect::FIELDS` order.
/// It must be pure, return a complete value, reconstruct omitted transient fields,
/// and preserve the reflected values exactly. No world or platform access is
/// provided. Do not register renderer handles, live tasks, or transient events.
pub trait SceneData: Reflect + Sized {
    /// Construct and validate one complete domain value before scene application.
    ///
    /// # Errors
    /// Rejects invalid domain values even when their scalar kinds are correct.
    fn from_scene_values(values: &[ReflectValue]) -> Result<Self, SceneValueError>;
}

pub(super) type ComponentConstructor =
    fn(&[ReflectValue]) -> Result<ComponentCommit, SceneValueError>;
pub(super) type ResourceConstructor =
    fn(&[ReflectValue]) -> Result<ResourceCommit, SceneValueError>;

/// Authoritative persistence adapters, separate from read-only reflection opt-in.
///
/// Register during construction. The registry is `Send + Sync` and contains no
/// world data. Readers and constructors execute synchronously on the caller's
/// thread. Unregistered world data is deliberately excluded from capture.
#[derive(Default)]
pub struct SceneRegistry {
    pub(super) reflection: ReflectionRegistry,
    pub(super) components: BTreeMap<&'static str, ComponentConstructor>,
    pub(super) resources: BTreeMap<&'static str, ResourceConstructor>,
}

impl SceneRegistry {
    /// Register an authoritative component's reflection and constructor together.
    ///
    /// # Errors
    /// Rejects duplicate type identifiers and invalid reflection metadata.
    pub fn register_component<T: SceneData>(&mut self) -> Result<(), SceneError> {
        self.reflection.register_component::<T>()?;
        self.components.insert(T::TYPE_NAME, |values| {
            let value = construct::<T>(values)?;
            Ok(Box::new(move |world, entity| {
                world.insert_component(entity, value);
            }))
        });
        Ok(())
    }

    /// Register an authoritative global resource's reflection and constructor.
    ///
    /// # Errors
    /// Rejects duplicate type identifiers and invalid reflection metadata.
    pub fn register_resource<T: SceneData>(&mut self) -> Result<(), SceneError> {
        self.reflection.register_resource::<T>()?;
        self.resources.insert(T::TYPE_NAME, |values| {
            let value = construct::<T>(values)?;
            Ok(Box::new(move |world| world.insert_resource(value)))
        });
        Ok(())
    }

    /// Read the metadata of the types explicitly registered for scene persistence.
    #[must_use]
    pub fn reflection(&self) -> &ReflectionRegistry {
        &self.reflection
    }
}

fn construct<T: SceneData>(values: &[ReflectValue]) -> Result<T, SceneValueError> {
    let value = T::from_scene_values(values)?;
    if value.reflect_values() != values {
        return Err(SceneValueError(
            "constructor did not preserve the persisted fields".into(),
        ));
    }
    Ok(value)
}
