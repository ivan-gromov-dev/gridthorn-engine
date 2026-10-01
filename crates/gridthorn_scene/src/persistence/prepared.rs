use gridthorn_world::{EntityId, SceneId, WorldAccess};

pub(super) type ComponentCommit =
    Box<dyn for<'world> FnOnce(&mut WorldAccess<'world>, EntityId) + Send + Sync>;
pub(super) type ResourceCommit =
    Box<dyn for<'world> FnOnce(&mut WorldAccess<'world>) + Send + Sync>;

/// Complete validated data awaiting one explicit world load boundary.
///
/// Dropping it discards prepared values without changing the world. `Send + Sync`;
/// commit is synchronous and invokes only engine-owned storage adapters, never
/// domain constructors. This transaction covers recoverable failures, not panics
/// in user destructors or allocation failure.
pub struct PreparedScene {
    pub(super) scene: SceneId,
    pub(super) entities: Vec<Vec<ComponentCommit>>,
    pub(super) resources: Vec<ResourceCommit>,
}

/// Applied scene identity and newly created runtime entities in document order.
pub struct SceneLoad {
    /// Scene whose owned entities were replaced.
    pub scene: SceneId,
    /// New runtime identifiers; previous scene identifiers are stale.
    pub entities: Vec<EntityId>,
}

impl PreparedScene {
    /// Atomically apply prepared values at an explicit load/reset boundary.
    ///
    /// Removes entities owned by this scene, recreates document entities, and
    /// overlays listed resources. Persistent entities, other scenes, and absent
    /// resources are preserved. No systems execute between these operations.
    /// This does not request a `SceneController` transition or run construction
    /// systems; application orchestration remains the caller's responsibility.
    #[must_use]
    pub fn commit(self, world: &mut WorldAccess<'_>) -> SceneLoad {
        world.despawn_scene(&self.scene);
        let mut entities = Vec::with_capacity(self.entities.len());
        for components in self.entities {
            let entity = world.spawn_empty_in_scene(self.scene.clone());
            for component in components {
                component(world, entity);
            }
            entities.push(entity);
        }
        for resource in self.resources {
            resource(world);
        }
        SceneLoad {
            scene: self.scene,
            entities,
        }
    }
}
