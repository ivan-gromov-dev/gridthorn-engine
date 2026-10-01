use crate::{
    FieldMetadata, Reflect, ReflectValue, ReflectionError, ReflectionRegistry, SceneId, ValueKind,
    WorldAccess,
};
use bevy_ecs::world::World;

struct Health(u64);
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

struct Invalid;
impl Reflect for Invalid {
    const TYPE_NAME: &'static str = "game.invalid";
    const FIELDS: &'static [FieldMetadata] = &[FieldMetadata {
        name: "value",
        kind: ValueKind::Float,
    }];
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![ReflectValue::Float(f64::NAN)]
    }
}

#[test]
fn inspects_live_components_and_resources_without_mutating_them() {
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    let mut registry = ReflectionRegistry::default();
    registry.register_component::<Health>().unwrap();
    registry.register_resource::<Health>().unwrap();
    let scene = SceneId::new("test").unwrap();
    let entity = world.spawn_in_scene(scene.clone(), Health(10));
    assert_eq!(
        registry
            .inspect_resource(&world, Health::TYPE_NAME)
            .unwrap(),
        None
    );
    world.insert_resource(Health(20));
    assert_eq!(
        registry
            .inspect_component(&world, entity, Health::TYPE_NAME)
            .unwrap(),
        Some(vec![ReflectValue::Unsigned(10)])
    );
    assert_eq!(
        registry
            .inspect_resource(&world, Health::TYPE_NAME)
            .unwrap(),
        Some(vec![ReflectValue::Unsigned(20)])
    );
    world.update_component(entity, |value: &mut Health| value.0 = 30);
    assert_eq!(
        registry
            .inspect_component(&world, entity, Health::TYPE_NAME)
            .unwrap(),
        Some(vec![ReflectValue::Unsigned(30)])
    );
    assert_eq!(world.read_resource(|value: &Health| value.0), Some(20));
    let unrelated = world.spawn(0_u32);
    assert_eq!(
        registry
            .inspect_component(&world, unrelated, Health::TYPE_NAME)
            .unwrap(),
        None
    );
    world.despawn_scene(&scene);
    assert_eq!(
        registry
            .inspect_component(&world, entity, Health::TYPE_NAME)
            .unwrap(),
        None
    );
}

#[test]
fn rejects_duplicates_unknown_types_and_invalid_values() {
    let mut registry = ReflectionRegistry::default();
    registry.register_resource::<Health>().unwrap();
    assert!(matches!(
        registry.register_resource::<Health>(),
        Err(ReflectionError::DuplicateType(_))
    ));
    registry.register_resource::<Invalid>().unwrap();
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    world.insert_resource(Invalid);
    assert!(matches!(
        registry.inspect_resource(&world, "unknown"),
        Err(ReflectionError::UnknownType(_))
    ));
    assert!(matches!(
        registry.inspect_resource(&world, Invalid::TYPE_NAME),
        Err(ReflectionError::InvalidValues(_))
    ));
    assert_eq!(
        registry
            .types()
            .map(|metadata| metadata.name)
            .collect::<Vec<_>>(),
        vec![Health::TYPE_NAME, Invalid::TYPE_NAME]
    );
}

struct DuplicateFields;
impl Reflect for DuplicateFields {
    const TYPE_NAME: &'static str = "duplicate";
    const FIELDS: &'static [FieldMetadata] = &[
        FieldMetadata {
            name: "x",
            kind: ValueKind::Bool,
        },
        FieldMetadata {
            name: "x",
            kind: ValueKind::Bool,
        },
    ];
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![]
    }
}
struct BadName;
impl Reflect for BadName {
    const TYPE_NAME: &'static str = "bad name";
    const FIELDS: &'static [FieldMetadata] = &[];
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![]
    }
}
struct WrongValues;
impl Reflect for WrongValues {
    const TYPE_NAME: &'static str = "wrong";
    const FIELDS: &'static [FieldMetadata] = Health::FIELDS;
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![ReflectValue::Bool(true)]
    }
}
#[test]
fn failed_registration_is_atomic_and_value_kinds_are_checked() {
    let mut registry = ReflectionRegistry::default();
    assert!(matches!(
        registry.register_component::<DuplicateFields>(),
        Err(ReflectionError::DuplicateField { .. })
    ));
    assert!(matches!(
        registry.register_component::<BadName>(),
        Err(ReflectionError::InvalidName { .. })
    ));
    assert_eq!(registry.types().count(), 0);
    registry.register_resource::<WrongValues>().unwrap();
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    world.insert_resource(WrongValues);
    assert!(matches!(
        registry.inspect_resource(&world, "wrong"),
        Err(ReflectionError::InvalidValues(_))
    ));
}

struct EmptyValues;
impl Reflect for EmptyValues {
    const TYPE_NAME: &'static str = "empty.values";
    const FIELDS: &'static [FieldMetadata] = Health::FIELDS;
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![]
    }
}
struct NameCollision;
impl Reflect for NameCollision {
    const TYPE_NAME: &'static str = Health::TYPE_NAME;
    const FIELDS: &'static [FieldMetadata] = &[];
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![]
    }
}
#[test]
fn rejects_identifier_collisions_and_missing_values() {
    let mut registry = ReflectionRegistry::default();
    registry.register_resource::<Health>().unwrap();
    assert!(matches!(
        registry.register_resource::<NameCollision>(),
        Err(ReflectionError::DuplicateType(_))
    ));
    assert_eq!(registry.types().count(), 1);
    registry.register_resource::<EmptyValues>().unwrap();
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    world.insert_resource(EmptyValues);
    assert!(matches!(
        registry.inspect_resource(&world, EmptyValues::TYPE_NAME),
        Err(ReflectionError::InvalidValues(_))
    ));
}
