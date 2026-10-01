/// Supported scalar field kinds in the provisional reflection contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueKind {
    /// Boolean flag.
    Bool,
    /// Signed integer.
    Integer,
    /// Unsigned integer.
    Unsigned,
    /// Finite floating-point number.
    Float,
    /// Owned UTF-8 text.
    Text,
}

/// Owned snapshot of a reflected scalar field.
#[derive(Clone, Debug, PartialEq)]
pub enum ReflectValue {
    /// Boolean flag.
    Bool(bool),
    /// Signed integer.
    Integer(i64),
    /// Unsigned integer.
    Unsigned(u64),
    /// Finite floating-point number.
    Float(f64),
    /// Owned UTF-8 text.
    Text(String),
}

impl ReflectValue {
    /// Return the scalar kind represented by this value.
    #[must_use]
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Bool(_) => ValueKind::Bool,
            Self::Integer(_) => ValueKind::Integer,
            Self::Unsigned(_) => ValueKind::Unsigned,
            Self::Float(_) => ValueKind::Float,
            Self::Text(_) => ValueKind::Text,
        }
    }
}

/// One named field in declaration order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldMetadata {
    /// Stable field name, independent of Rust type names.
    pub name: &'static str,
    /// Expected scalar value kind.
    pub kind: ValueKind,
}

/// Explicit opt-in to read-only reflection, independent of the ECS backend.
///
/// Implementations return one value per field in metadata order. Runtime-only
/// fields should be omitted. Callbacks execute synchronously on the caller's
/// thread; they must not mutate authoritative data or perform external work.
pub trait Reflect: Send + Sync + 'static {
    /// Stable application-owned identifier; renaming may break future persistence.
    const TYPE_NAME: &'static str;
    /// Metadata ordered exactly like the values returned by `reflect_values`.
    const FIELDS: &'static [FieldMetadata];
    /// Copy exposed fields into an owned inspection snapshot.
    fn reflect_values(&self) -> Vec<ReflectValue>;
}
