use bevy_ecs::world::World;

use crate::{SceneId, scene::SceneOwner};

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

    /// Spawn an entity owned by one scene.
    ///
    /// Scene-owned entities are removed together when that scene exits. Plain
    /// entities created with [`Self::spawn`] remain persistent across scenes.
    pub fn spawn_in_scene<T>(&mut self, scene: SceneId, component: T) -> EntityId
    where
        T: Send + Sync + 'static,
    {
        EntityId(
            self.backend
                .spawn((StoredComponent(component), SceneOwner(scene)))
                .id(),
        )
    }

    /// Remove every entity owned by a scene and return the number removed.
    pub fn despawn_scene(&mut self, scene: &SceneId) -> usize {
        let mut query = self
            .backend
            .query::<(bevy_ecs::entity::Entity, &SceneOwner)>();
        let entities = query
            .iter(self.backend)
            .filter_map(|(entity, owner)| (owner.0 == *scene).then_some(entity))
            .collect::<Vec<_>>();
        entities
            .into_iter()
            .filter(|entity| self.backend.despawn(*entity))
            .count()
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

    /// Read one component through a short-lived callback when it exists.
    pub fn read_component<T, R>(&self, entity: EntityId, read: impl FnOnce(&T) -> R) -> Option<R>
    where
        T: Send + Sync + 'static,
    {
        self.backend
            .get::<StoredComponent<T>>(entity.0)
            .map(|component| read(&component.0))
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
        self.update_resource_with(|resource| update(resource))
            .is_some()
    }

    /// Mutate a typed resource and return a value produced by the callback.
    pub fn update_resource_with<T, R>(&mut self, update: impl FnOnce(&mut T) -> R) -> Option<R>
    where
        T: Send + Sync + 'static,
    {
        self.backend
            .get_resource_mut::<StoredResource<T>>()
            .map(|mut resource| update(&mut resource.0))
    }
}
