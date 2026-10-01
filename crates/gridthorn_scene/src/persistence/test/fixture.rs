use gridthorn_world::{FieldMetadata, Reflect, ReflectValue, SceneId, ValueKind};

use crate::{SceneData, SceneRegistry, SceneValueError};

#[derive(Debug, PartialEq)]
pub(super) struct Health(pub u64);

impl Reflect for Health {
    const TYPE_NAME: &'static str = "game.health";
    const FIELDS: &'static [FieldMetadata] = &[FieldMetadata {
        name: "points",
        kind: ValueKind::Unsigned,
    }];

    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![ReflectValue::Unsigned(self.0)]
    }
}

impl SceneData for Health {
    fn from_scene_values(values: &[ReflectValue]) -> Result<Self, SceneValueError> {
        match values {
            [ReflectValue::Unsigned(value)] if *value > 0 => Ok(Self(*value)),
            _ => Err(SceneValueError("health points must be positive".into())),
        }
    }
}

pub(super) fn registry() -> SceneRegistry {
    let mut registry = SceneRegistry::default();
    registry.register_component::<Health>().unwrap();
    registry.register_resource::<Health>().unwrap();
    registry
}

pub(super) fn scene() -> SceneId {
    SceneId::new("level.one").unwrap()
}
