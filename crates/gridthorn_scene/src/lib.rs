//! Provisional scalar scene persistence through engine-owned world contracts.

mod persistence;

pub use persistence::{
    PreparedScene, SceneData, SceneDocument, SceneEntityData, SceneError, SceneLoad,
    SceneMigrations, SceneRecord, SceneRegistry, SceneScalar, SceneValueError,
};
