use gridthorn_world::{FieldMetadata, Reflect, ReflectValue, ScheduleBuilder, ValueKind};

use crate::{SceneData, SceneDocument, SceneRegistry, SceneValueError};

use super::{Health, registry, scene};

#[test]
fn capture_encode_decode_prepare_commit_preserves_registered_data_and_scope() {
    let mut registry = registry();
    registry.register_component::<Scalars>().unwrap();
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    let old = world.spawn_in_scene(scene(), Health(u64::MAX));
    world.insert_component(old, 7_u32);
    world.insert_component(old, Scalars);
    world.spawn_empty_in_scene(scene());
    let persistent = world.spawn(Health(40));
    let other_scene = gridthorn_world::SceneId::new("other").unwrap();
    let other = world.spawn_in_scene(other_scene.clone(), Health(50));
    world.insert_resource(Health(100));
    world.insert_resource(9_u32);
    let document = registry.capture(&mut world, &scene()).unwrap();
    assert_eq!(document.entities.len(), 2);
    assert_eq!(document.entities[0].components.len(), 2);
    let text = document.to_toml().unwrap();
    assert_eq!(
        text,
        registry
            .capture(&mut world, &scene())
            .unwrap()
            .to_toml()
            .unwrap()
    );
    assert!(!text.contains("StoredComponent"));
    assert_eq!(SceneDocument::from_toml(&text).unwrap(), document);
    let prepared = registry
        .prepare(&SceneDocument::from_toml(&text).unwrap())
        .unwrap();
    assert_eq!(
        world.read_component(old, |value: &Health| value.0),
        Some(u64::MAX)
    );
    world.update_resource(|value: &mut Health| value.0 = 200);
    let loaded = prepared.commit(&mut world);
    assert_eq!(loaded.scene, scene());
    assert_eq!(loaded.entities.len(), 2);
    assert_eq!(world.read_component(old, |value: &Health| value.0), None);
    assert_eq!(
        world.read_component(loaded.entities[0], |value: &Health| value.0),
        Some(u64::MAX)
    );
    assert_eq!(
        world.read_component(loaded.entities[0], |value: &u32| *value),
        None
    );
    assert_eq!(
        world.read_component(loaded.entities[0], |value: &Scalars| value.reflect_values()),
        Some(Scalars.reflect_values())
    );
    assert_eq!(
        world.read_component(persistent, |value: &Health| value.0),
        Some(40)
    );
    assert_eq!(
        world.read_component(other, |value: &Health| value.0),
        Some(50)
    );
    assert_eq!(world.read_resource(|value: &Health| value.0), Some(100));
    assert_eq!(world.read_resource(|value: &u32| *value), Some(9));
    assert_eq!(world.scene_entities(&scene()).len(), 2);
    assert_eq!(world.scene_entities(&other_scene), vec![other]);
}

#[test]
fn dropped_preparation_and_empty_document_preserve_resources() {
    let registry = registry();
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    let entity = world.spawn_in_scene(scene(), Health(10));
    world.insert_resource(Health(20));
    let document = registry.capture(&mut world, &scene()).unwrap();
    drop(registry.prepare(&document).unwrap());
    assert_eq!(
        world.read_component(entity, |value: &Health| value.0),
        Some(10)
    );
    let loaded = registry
        .prepare(&SceneDocument::new(&scene()))
        .unwrap()
        .commit(&mut world);
    assert_eq!(loaded.entities, []);
    assert_eq!(world.read_component(entity, |value: &Health| value.0), None);
    assert_eq!(world.read_resource(|value: &Health| value.0), Some(20));
}

#[derive(Debug, PartialEq)]
struct Scalars;
impl Reflect for Scalars {
    const TYPE_NAME: &'static str = "game.scalars";
    const FIELDS: &'static [FieldMetadata] = &[
        FieldMetadata {
            name: "flag",
            kind: ValueKind::Bool,
        },
        FieldMetadata {
            name: "integer",
            kind: ValueKind::Integer,
        },
        FieldMetadata {
            name: "float",
            kind: ValueKind::Float,
        },
        FieldMetadata {
            name: "text",
            kind: ValueKind::Text,
        },
    ];
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![
            ReflectValue::Bool(true),
            ReflectValue::Integer(i64::MIN),
            ReflectValue::Float(-0.5),
            ReflectValue::Text("雪\n\"text\"".into()),
        ]
    }
}
impl SceneData for Scalars {
    fn from_scene_values(_: &[ReflectValue]) -> Result<Self, SceneValueError> {
        Ok(Self)
    }
}

#[test]
fn all_scalar_kinds_and_unicode_roundtrip_through_toml() {
    let mut registry = SceneRegistry::default();
    registry.register_resource::<Scalars>().unwrap();
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    world.insert_resource(Scalars);
    let document = registry.capture(&mut world, &scene()).unwrap();
    let decoded = SceneDocument::from_toml(&document.to_toml().unwrap()).unwrap();
    assert_eq!(decoded, document);
    let _loaded = registry.prepare(&decoded).unwrap().commit(&mut world);
    assert_eq!(
        world.read_resource(|value: &Scalars| value.reflect_values()),
        Some(Scalars.reflect_values())
    );
}
