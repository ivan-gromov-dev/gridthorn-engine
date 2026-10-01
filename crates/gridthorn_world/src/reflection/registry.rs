use std::{any::TypeId, collections::BTreeMap};

use super::{FieldMetadata, Reflect, ReflectValue, ReflectionError};
use crate::{EntityId, WorldAccess};

/// Storage location exposed by a registered reflection adapter.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReflectionRole {
    /// A component attached to an entity.
    Component,
    /// A singleton world resource.
    Resource,
}

/// Registered metadata for one storage role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReflectedType {
    /// Stable application-owned type identifier.
    pub name: &'static str,
    /// Component or resource storage.
    pub role: ReflectionRole,
    /// Fields in declaration order.
    pub fields: &'static [FieldMetadata],
}

type Reader = for<'a, 'b> fn(&'a WorldAccess<'b>, Option<EntityId>) -> Option<Vec<ReflectValue>>;

struct Registration {
    metadata: ReflectedType,
    rust_type: TypeId,
    read: Reader,
}

/// Explicit, read-only component/resource adapters with deterministic enumeration.
///
/// Registration is intended for application construction. This registry contains
/// no world storage and can be shared across threads (`Send + Sync`). Inspection
/// borrows the caller's world and returns owned values. No unregistered data is
/// exposed, and no mutation or serialization is performed.
#[derive(Default)]
pub struct ReflectionRegistry {
    registrations: BTreeMap<(ReflectionRole, &'static str), Registration>,
}

impl ReflectionRegistry {
    /// Register a component type without changing world storage.
    ///
    /// # Errors
    /// Rejects invalid metadata and duplicate identifiers or Rust types.
    pub fn register_component<T: Reflect>(&mut self) -> Result<(), ReflectionError> {
        self.register::<T>(ReflectionRole::Component, |world, entity| {
            world.read_component::<T, _>(entity?, Reflect::reflect_values)
        })
    }

    /// Register a resource type without changing world storage.
    ///
    /// # Errors
    /// Rejects invalid metadata and duplicate identifiers or Rust types.
    pub fn register_resource<T: Reflect>(&mut self) -> Result<(), ReflectionError> {
        self.register::<T>(ReflectionRole::Resource, |world, _| {
            world.read_resource::<T, _>(Reflect::reflect_values)
        })
    }

    fn register<T: Reflect>(
        &mut self,
        role: ReflectionRole,
        read: Reader,
    ) -> Result<(), ReflectionError> {
        for name in std::iter::once(T::TYPE_NAME).chain(T::FIELDS.iter().map(|field| field.name)) {
            if name.is_empty() || name.chars().any(char::is_whitespace) {
                return Err(ReflectionError::InvalidName {
                    type_name: T::TYPE_NAME.into(),
                    name: name.into(),
                });
            }
        }
        let mut names = std::collections::BTreeSet::new();
        for field in T::FIELDS {
            if !names.insert(field.name) {
                return Err(ReflectionError::DuplicateField {
                    type_name: T::TYPE_NAME.into(),
                    field: field.name.into(),
                });
            }
        }
        if self.registrations.values().any(|entry| {
            entry.metadata.role == role
                && (entry.metadata.name == T::TYPE_NAME || entry.rust_type == TypeId::of::<T>())
        }) {
            return Err(ReflectionError::DuplicateType(T::TYPE_NAME.into()));
        }
        self.registrations.insert(
            (role, T::TYPE_NAME),
            Registration {
                metadata: ReflectedType {
                    name: T::TYPE_NAME,
                    role,
                    fields: T::FIELDS,
                },
                rust_type: TypeId::of::<T>(),
                read,
            },
        );
        Ok(())
    }

    /// Enumerate metadata ordered by role, then stable type identifier.
    pub fn types(&self) -> impl Iterator<Item = &ReflectedType> {
        self.registrations.values().map(|entry| &entry.metadata)
    }

    /// Inspect a registered component; missing entities/components return `None`.
    ///
    /// # Errors
    /// Rejects unknown types and callback values inconsistent with metadata.
    pub fn inspect_component(
        &self,
        world: &WorldAccess<'_>,
        entity: EntityId,
        name: &str,
    ) -> Result<Option<Vec<ReflectValue>>, ReflectionError> {
        self.inspect(world, Some(entity), ReflectionRole::Component, name)
    }

    /// Inspect a registered resource; absent resources return `None`.
    ///
    /// # Errors
    /// Rejects unknown types and callback values inconsistent with metadata.
    pub fn inspect_resource(
        &self,
        world: &WorldAccess<'_>,
        name: &str,
    ) -> Result<Option<Vec<ReflectValue>>, ReflectionError> {
        self.inspect(world, None, ReflectionRole::Resource, name)
    }

    fn inspect(
        &self,
        world: &WorldAccess<'_>,
        entity: Option<EntityId>,
        role: ReflectionRole,
        name: &str,
    ) -> Result<Option<Vec<ReflectValue>>, ReflectionError> {
        let entry = self
            .registrations
            .iter()
            .find_map(|((registered_role, registered_name), entry)| {
                (*registered_role == role && *registered_name == name).then_some(entry)
            })
            .ok_or_else(|| ReflectionError::UnknownType(name.into()))?;
        let Some(values) = (entry.read)(world, entity) else {
            return Ok(None);
        };
        if values.len() != entry.metadata.fields.len()
            || values
                .iter()
                .zip(entry.metadata.fields)
                .any(|(value, field)| {
                    value.kind() != field.kind
                        || matches!(value, ReflectValue::Float(number) if !number.is_finite())
                })
        {
            return Err(ReflectionError::InvalidValues(name.into()));
        }
        Ok(Some(values))
    }
}
