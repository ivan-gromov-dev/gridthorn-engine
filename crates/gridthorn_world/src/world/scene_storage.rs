use crate::{EntityId, SceneId, WorldAccess, scene::SceneOwner};

use super::StoredComponent;

impl WorldAccess<'_> {
    /// Enumerate scene-owned entities in ascending runtime storage index order.
    ///
    /// Ordering is deterministic for unchanged storage. Runtime identifiers are
    /// not persistence identifiers and must not be stored in scene documents.
    pub fn scene_entities(&mut self, scene: &SceneId) -> Vec<EntityId> {
        let mut query = self
            .backend
            .query::<(bevy_ecs::entity::Entity, &SceneOwner)>();
        let mut entities: Vec<_> = query
            .iter(self.backend)
            .filter_map(|(entity, owner)| (owner.0 == *scene).then_some(EntityId(entity)))
            .collect();
        entities.sort_by_key(|entity| entity.0.index_u32());
        entities
    }

    /// Create an empty scene-owned entity during explicit construction/load.
    pub fn spawn_empty_in_scene(&mut self, scene: SceneId) -> EntityId {
        EntityId(self.backend.spawn(SceneOwner(scene)).id())
    }

    /// Insert or replace a domain component; return false for a stale entity.
    pub fn insert_component<T: Send + Sync + 'static>(
        &mut self,
        entity: EntityId,
        component: T,
    ) -> bool {
        let Ok(mut entity) = self.backend.get_entity_mut(entity.0) else {
            return false;
        };
        entity.insert(StoredComponent(component));
        true
    }
}
