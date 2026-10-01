use std::collections::BTreeMap;

use super::{SceneDocument, SceneError, SceneValueError};

type Migration = fn(SceneDocument) -> Result<SceneDocument, SceneValueError>;

/// Ordered, explicit schema transformations, independent of world storage.
///
/// Callbacks must be pure and deterministic: no ambient time, filesystem,
/// randomness, or global mutation. They advance exactly one schema version and
/// must retain valid format/engine/scene metadata. Schema 1 currently needs no
/// built-in migration; projects can explicitly adapt structurally compatible
/// older documents. `Send + Sync`; callbacks execute on the caller's thread.
#[derive(Default)]
pub struct SceneMigrations {
    steps: BTreeMap<u32, Migration>,
}

impl SceneMigrations {
    /// Register a migration from `version` to `version + 1`.
    ///
    /// # Errors
    /// Rejects repeated source versions and sources at/above the current schema.
    pub fn register(&mut self, version: u32, migration: Migration) -> Result<(), SceneError> {
        if version >= 1 || self.steps.contains_key(&version) {
            return Err(SceneError::MigrationRegistration(version));
        }
        self.steps.insert(version, migration);
        Ok(())
    }

    /// Validate compatibility and transform an owned document to schema 1.
    ///
    /// # Errors
    /// Rejects future versions, missing steps, failed callbacks, incorrect version
    /// advances, and invalid resulting envelope metadata. No world is accessed.
    pub fn upgrade(&self, mut document: SceneDocument) -> Result<SceneDocument, SceneError> {
        document.validate_envelope()?;
        if document.schema_version > 1 {
            return Err(SceneError::Schema(document.schema_version));
        }
        while document.schema_version < 1 {
            let version = document.schema_version;
            let migration = self
                .steps
                .get(&version)
                .ok_or(SceneError::Schema(version))?;
            document = migration(document).map_err(|error| SceneError::Migration {
                version,
                reason: error.to_string(),
            })?;
            if document.schema_version != version + 1 {
                return Err(SceneError::Migration {
                    version,
                    reason: "callback must advance exactly one version".into(),
                });
            }
            document.validate_envelope()?;
        }
        Ok(document)
    }
}
