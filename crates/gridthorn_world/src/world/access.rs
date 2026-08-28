use bevy_ecs::world::World;

use super::{EntityId, StoredComponent, StoredResource};

/// Borrowed access to Gridthorn-owned ECS state.
pub struct WorldAccess<'world> {
    pub(crate) backend: &'world mut World,
}

impl WorldAccess<'_> {
    /// Spawn an entity with one domain component.
    pub fn spawn<T>(&mut self, component: T) -> EntityId
    where
        T: Send + Sync + 'static,
    {
        EntityId(self.backend.spawn(StoredComponent(component)).id())
    }

    /// Apply a mutation to one component when the entity and type exist.
    pub fn update_component<T>(&mut self, entity: EntityId, update: impl FnOnce(&mut T)) -> bool
    where
        T: Send + Sync + 'static,
    {
        let Some(mut component) = self.backend.get_mut::<StoredComponent<T>>(entity.0) else {
            return false;
        };
        update(&mut component.0);
        true
    }

    /// Visit every component of one domain type.
    pub fn for_each_component_mut<T>(&mut self, mut visit: impl FnMut(EntityId, &mut T))
    where
        T: Send + Sync + 'static,
    {
        let mut query = self
            .backend
            .query::<(bevy_ecs::entity::Entity, &mut StoredComponent<T>)>();
        for (entity, mut component) in query.iter_mut(self.backend) {
            visit(EntityId(entity), &mut component.0);
        }
    }

    /// Insert or replace a typed world resource.
    pub fn insert_resource<T>(&mut self, resource: T)
    where
        T: Send + Sync + 'static,
    {
        self.backend.insert_resource(StoredResource(resource));
    }

    /// Read a typed resource through a short-lived callback.
    pub fn read_resource<T, R>(&self, read: impl FnOnce(&T) -> R) -> Option<R>
    where
        T: Send + Sync + 'static,
    {
        self.backend
            .get_resource::<StoredResource<T>>()
            .map(|resource| read(&resource.0))
    }

    /// Mutate a typed resource through a short-lived callback.
    pub fn update_resource<T>(&mut self, update: impl FnOnce(&mut T)) -> bool
    where
        T: Send + Sync + 'static,
    {
        let Some(mut resource) = self.backend.get_resource_mut::<StoredResource<T>>() else {
            return false;
        };
        update(&mut resource.0);
        true
    }
}
