use gridthorn_world::ReflectValue;
use serde::{Deserialize, Serialize};

/// Portable scalar wire values; unsigned integers use decimal text for full u64 range.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum SceneScalar {
    /// Boolean flag.
    Bool(bool),
    /// Signed 64-bit integer.
    Integer(i64),
    /// Decimal representation of an unsigned 64-bit integer.
    Unsigned(String),
    /// Finite 64-bit floating-point value.
    Float(f64),
    /// Owned UTF-8 text.
    Text(String),
}

impl From<ReflectValue> for SceneScalar {
    fn from(value: ReflectValue) -> Self {
        match value {
            ReflectValue::Bool(value) => Self::Bool(value),
            ReflectValue::Integer(value) => Self::Integer(value),
            ReflectValue::Unsigned(value) => Self::Unsigned(value.to_string()),
            ReflectValue::Float(value) => Self::Float(value),
            ReflectValue::Text(value) => Self::Text(value),
        }
    }
}

impl SceneScalar {
    pub(super) fn reflected(&self) -> Result<ReflectValue, String> {
        match self {
            Self::Bool(value) => Ok(ReflectValue::Bool(*value)),
            Self::Integer(value) => Ok(ReflectValue::Integer(*value)),
            Self::Unsigned(value) => {
                let parsed = value
                    .parse::<u64>()
                    .map_err(|_| "invalid unsigned integer".to_owned())?;
                if parsed.to_string() != *value {
                    return Err("unsigned integer must use canonical decimal text".into());
                }
                Ok(ReflectValue::Unsigned(parsed))
            }
            Self::Float(value) if value.is_finite() => Ok(ReflectValue::Float(*value)),
            Self::Float(_) => Err("floating-point value must be finite".into()),
            Self::Text(value) => Ok(ReflectValue::Text(value.clone())),
        }
    }
}
