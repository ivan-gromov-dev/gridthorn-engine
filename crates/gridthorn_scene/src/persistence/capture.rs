use gridthorn_world::{ReflectionRole, SceneId, WorldAccess};

use super::{SceneDocument, SceneEntityData, SceneError, SceneRecord, SceneRegistry};

impl SceneRegistry {
    /// Snapshot registered authoritative fields of one scene and present resources.
    ///
    /// Entity order is normalized by runtime identifier; type and field ordering
    /// uses stable names. Unregistered fields/data and backend entity IDs are not
    /// persisted. Invoke at a fixed boundary or explicit load/reset boundary.
    ///
    /// # Errors
    /// Rejects reflection callbacks inconsistent with metadata or invalid scalars.
    pub fn capture(
        &self,
        world: &mut WorldAccess<'_>,
        scene: &SceneId,
    ) -> Result<SceneDocument, SceneError> {
        let mut document = SceneDocument::new(scene);
        for entity in world.scene_entities(scene) {
            let mut components = Vec::new();
            for metadata in self
                .reflection
                .types()
                .filter(|metadata| metadata.role == ReflectionRole::Component)
            {
                if let Some(values) =
                    self.reflection
                        .inspect_component(world, entity, metadata.name)?
                {
                    components.push(SceneRecord {
                        type_name: metadata.name.into(),
                        fields: metadata
                            .fields
                            .iter()
                            .zip(values)
                            .map(|(field, value)| (field.name.into(), value.into()))
                            .collect(),
                    });
                }
            }
            document.entities.push(SceneEntityData { components });
        }
        for metadata in self
            .reflection
            .types()
            .filter(|metadata| metadata.role == ReflectionRole::Resource)
        {
            if let Some(values) = self.reflection.inspect_resource(world, metadata.name)? {
                document.resources.push(SceneRecord {
                    type_name: metadata.name.into(),
                    fields: metadata
                        .fields
                        .iter()
                        .zip(values)
                        .map(|(field, value)| (field.name.into(), value.into()))
                        .collect(),
                });
            }
        }
        Ok(document)
    }
}
