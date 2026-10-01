use thiserror::Error;

/// Domain constructor rejection during scene preparation.
#[derive(Debug, Error, PartialEq, Eq)]
#[error("{0}")]
pub struct SceneValueError(pub String);

/// Contextual, recoverable scene persistence failures.
#[derive(Debug, Error)]
pub enum SceneError {
    /// Invalid TOML or incompatible document structure.
    #[error("invalid scene document: {0}")]
    Decode(String),
    /// The document could not be represented as TOML.
    #[error("cannot encode scene document: {0}")]
    Encode(String),
    /// The format identifier is not supported.
    #[error("unsupported scene format '{0}' (expected gridthorn.scene)")]
    Format(String),
    /// Future schema version or a missing migration.
    #[error("unsupported scene schema {0}; current schema is 1")]
    Schema(u32),
    /// Invalid or unsatisfied engine compatibility requirement.
    #[error("scene requires engine '{0}'; running engine is {version}", version = env!("CARGO_PKG_VERSION"))]
    Engine(String),
    /// Invalid scene identifier.
    #[error(transparent)]
    SceneId(#[from] gridthorn_world::SceneIdError),
    /// Invalid reflection registration or snapshot.
    #[error(transparent)]
    Reflection(#[from] gridthorn_world::ReflectionError),
    /// Required type is not registered for the requested storage role.
    #[error("{location}: required {role} type '{type_name}' is not registered")]
    UnknownType {
        location: String,
        role: &'static str,
        type_name: String,
    },
    /// Duplicated records would silently overwrite each other.
    #[error("{location}: duplicate type '{type_name}'")]
    Duplicate { location: String, type_name: String },
    /// Named field or scalar representation does not match registered metadata.
    #[error("{location}: type '{type_name}' has invalid fields: {reason}")]
    Fields {
        location: String,
        type_name: String,
        reason: String,
    },
    /// Domain construction failed before any world changes.
    #[error("{location}: cannot construct type '{type_name}': {source}")]
    Construct {
        location: String,
        type_name: String,
        #[source]
        source: SceneValueError,
    },
    /// A migration was registered twice or has an invalid source version.
    #[error("invalid or duplicate scene migration from schema {0}")]
    MigrationRegistration(u32),
    /// A migration failed or did not advance exactly one schema version.
    #[error("scene migration from schema {version} failed: {reason}")]
    Migration { version: u32, reason: String },
}
