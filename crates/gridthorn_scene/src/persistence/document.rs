use std::collections::BTreeMap;

use gridthorn_world::SceneId;
use serde::{Deserialize, Serialize};

use super::{SceneError, SceneScalar};

/// Named scalar fields belonging to one required registered type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneRecord {
    /// Stable reflection type identifier.
    pub type_name: String,
    /// Explicit field names, serialized in lexical order.
    pub fields: BTreeMap<String, SceneScalar>,
}

/// One entity's components; entity references are not part of schema 1.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneEntityData {
    /// Required components, ordered by stable type identifier on capture.
    pub components: Vec<SceneRecord>,
}

/// Provisional schema 1 scene document. Validation occurs before world application.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneDocument {
    /// Must be `gridthorn.scene`.
    pub format: String,
    /// Explicit schema version, currently 1.
    pub schema_version: u32,
    /// `SemVer` requirement for the engine; capture defaults to the exact release.
    pub engine: String,
    /// Scene identifier whose entities are replaced when the document is loaded.
    pub scene: String,
    /// Entities in document order; no backend identifiers are persisted.
    pub entities: Vec<SceneEntityData>,
    /// Explicit global resource overlays; absent resources remain unchanged.
    pub resources: Vec<SceneRecord>,
}

impl SceneDocument {
    /// Create an empty current-schema document for a scene and this engine release.
    #[must_use]
    pub fn new(scene: &SceneId) -> Self {
        Self {
            format: "gridthorn.scene".into(),
            schema_version: 1,
            engine: format!("={}", env!("CARGO_PKG_VERSION")),
            scene: scene.as_str().into(),
            entities: Vec::new(),
            resources: Vec::new(),
        }
    }

    /// Parse strict TOML structure without constructing or changing world data.
    ///
    /// # Errors
    /// Returns a contextual decode error for malformed or unknown document fields.
    pub fn from_toml(source: &str) -> Result<Self, SceneError> {
        toml::from_str(source).map_err(|error| SceneError::Decode(error.to_string()))
    }

    /// Encode deterministic TOML for this ordered document.
    ///
    /// # Errors
    /// Rejects invalid envelope metadata, unsupported schemas, and invalid scalars.
    pub fn to_toml(&self) -> Result<String, SceneError> {
        self.validate_envelope()?;
        if self.schema_version != 1 {
            return Err(SceneError::Schema(self.schema_version));
        }
        for (index, entity) in self.entities.iter().enumerate() {
            for record in &entity.components {
                validate_scalars(record, &format!("entity {index}"))?;
            }
        }
        for record in &self.resources {
            validate_scalars(record, "resources")?;
        }
        toml::to_string_pretty(self).map_err(|error| SceneError::Encode(error.to_string()))
    }

    pub(super) fn validate_envelope(&self) -> Result<SceneId, SceneError> {
        if self.format != "gridthorn.scene" {
            return Err(SceneError::Format(self.format.clone()));
        }
        let requirement = semver::VersionReq::parse(&self.engine)
            .map_err(|_| SceneError::Engine(self.engine.clone()))?;
        let version = semver::Version::parse(env!("CARGO_PKG_VERSION"))
            .map_err(|_| SceneError::Engine(self.engine.clone()))?;
        if !requirement.matches(&version) {
            return Err(SceneError::Engine(self.engine.clone()));
        }
        Ok(SceneId::new(self.scene.clone())?)
    }
}

fn validate_scalars(record: &SceneRecord, location: &str) -> Result<(), SceneError> {
    for (name, value) in &record.fields {
        value.reflected().map_err(|reason| SceneError::Fields {
            location: location.into(),
            type_name: record.type_name.clone(),
            reason: format!("field '{name}': {reason}"),
        })?;
    }
    Ok(())
}
