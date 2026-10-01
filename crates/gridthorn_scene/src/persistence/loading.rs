use std::collections::BTreeSet;

use gridthorn_world::{ReflectValue, ReflectionRole};

use super::{PreparedScene, SceneDocument, SceneError, SceneRecord, SceneRegistry};

impl SceneRegistry {
    /// Validate a current-schema document and construct all values off-world.
    ///
    /// # Errors
    /// Rejects incompatible metadata, unknown/duplicate required types, unknown
    /// or missing fields, wrong scalar kinds, and domain constructor failures.
    /// Every failure leaves the caller's world untouched.
    pub fn prepare(&self, document: &SceneDocument) -> Result<PreparedScene, SceneError> {
        let scene = document.validate_envelope()?;
        if document.schema_version != 1 {
            return Err(SceneError::Schema(document.schema_version));
        }
        let mut entities = Vec::with_capacity(document.entities.len());
        for (index, entity) in document.entities.iter().enumerate() {
            let location = format!("entity {index}");
            reject_duplicates(&entity.components, &location)?;
            let mut components = Vec::with_capacity(entity.components.len());
            for record in &entity.components {
                let constructor = self
                    .components
                    .get(record.type_name.as_str())
                    .ok_or_else(|| unknown(record, &location, "component"))?;
                let values = self.values(record, ReflectionRole::Component, &location)?;
                components.push(
                    constructor(&values).map_err(|source| SceneError::Construct {
                        location: location.clone(),
                        type_name: record.type_name.clone(),
                        source,
                    })?,
                );
            }
            entities.push(components);
        }
        reject_duplicates(&document.resources, "resources")?;
        let mut resources = Vec::with_capacity(document.resources.len());
        for record in &document.resources {
            let constructor = self
                .resources
                .get(record.type_name.as_str())
                .ok_or_else(|| unknown(record, "resources", "resource"))?;
            let values = self.values(record, ReflectionRole::Resource, "resources")?;
            resources.push(
                constructor(&values).map_err(|source| SceneError::Construct {
                    location: "resources".into(),
                    type_name: record.type_name.clone(),
                    source,
                })?,
            );
        }
        Ok(PreparedScene {
            scene,
            entities,
            resources,
        })
    }

    fn values(
        &self,
        record: &SceneRecord,
        role: ReflectionRole,
        location: &str,
    ) -> Result<Vec<ReflectValue>, SceneError> {
        let metadata = self
            .reflection
            .types()
            .find(|metadata| metadata.role == role && metadata.name == record.type_name)
            .ok_or_else(|| unknown(record, location, "registered"))?;
        let invalid = |reason: String| SceneError::Fields {
            location: location.into(),
            type_name: record.type_name.clone(),
            reason,
        };
        for name in record.fields.keys() {
            if !metadata.fields.iter().any(|field| field.name == name) {
                return Err(invalid(format!("unknown field '{name}'")));
            }
        }
        metadata
            .fields
            .iter()
            .map(|field| {
                let value = record
                    .fields
                    .get(field.name)
                    .ok_or_else(|| invalid(format!("missing field '{}'", field.name)))?;
                let value = value
                    .reflected()
                    .map_err(|reason| invalid(format!("field '{}': {reason}", field.name)))?;
                if value.kind() != field.kind {
                    return Err(invalid(format!(
                        "wrong scalar kind for field '{}'",
                        field.name
                    )));
                }
                Ok(value)
            })
            .collect()
    }
}

fn reject_duplicates(records: &[SceneRecord], location: &str) -> Result<(), SceneError> {
    let mut names = BTreeSet::new();
    for record in records {
        if !names.insert(&record.type_name) {
            return Err(SceneError::Duplicate {
                location: location.into(),
                type_name: record.type_name.clone(),
            });
        }
    }
    Ok(())
}

fn unknown(record: &SceneRecord, location: &str, role: &'static str) -> SceneError {
    SceneError::UnknownType {
        location: location.into(),
        role,
        type_name: record.type_name.clone(),
    }
}
