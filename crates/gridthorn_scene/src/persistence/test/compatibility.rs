use gridthorn_world::{Reflect, ReflectValue, ScheduleBuilder};

use crate::{SceneData, SceneDocument, SceneError, SceneScalar, SceneValueError};

use super::{Health, registry, scene};

#[test]
fn invalid_envelopes_are_rejected_before_loading() {
    let registry = registry();
    let mut document = SceneDocument::new(&scene());
    document.format = "other".into();
    assert!(matches!(
        registry.prepare(&document),
        Err(SceneError::Format(_))
    ));
    document = SceneDocument::new(&scene());
    document.schema_version = 2;
    assert!(matches!(
        registry.prepare(&document),
        Err(SceneError::Schema(2))
    ));
    document.schema_version = 0;
    assert!(matches!(
        registry.prepare(&document),
        Err(SceneError::Schema(0))
    ));
    for requirement in ["=99.0.0", "not-semver"] {
        document = SceneDocument::new(&scene());
        document.engine = requirement.into();
        assert!(matches!(
            registry.prepare(&document),
            Err(SceneError::Engine(_))
        ));
    }
    document = SceneDocument::new(&scene());
    document.scene = " ".into();
    assert!(matches!(
        registry.prepare(&document),
        Err(SceneError::SceneId(_))
    ));
    assert!(matches!(
        SceneDocument::from_toml("not a document"),
        Err(SceneError::Decode(_))
    ));
    let text = SceneDocument::new(&scene()).to_toml().unwrap();
    assert!(matches!(
        SceneDocument::from_toml(&(text + "\nunknown = true\n")),
        Err(SceneError::Decode(_))
    ));
}

#[test]
fn late_failures_preserve_scene_entities_and_every_resource() {
    let registry = registry();
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    let entity = world.spawn_in_scene(scene(), Health(10));
    world.insert_resource(Health(20));
    let baseline = registry.capture(&mut world, &scene()).unwrap();
    let mut invalid = baseline.clone();
    invalid.resources[0]
        .fields
        .insert("points".into(), SceneScalar::Unsigned("0".into()));
    let error = registry.prepare(&invalid).err().unwrap();
    assert!(matches!(error, SceneError::Construct { .. }));
    assert!(error.to_string().contains("resources"));
    assert!(error.to_string().contains("game.health"));
    assert!(error.to_string().contains("positive"));
    assert_eq!(registry.capture(&mut world, &scene()).unwrap(), baseline);
    assert_eq!(world.scene_entities(&scene()), vec![entity]);
}

#[test]
fn unknown_types_duplicate_records_and_invalid_fields_fail_contextually() {
    let registry = registry();
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    world.spawn_in_scene(scene(), Health(10));
    world.insert_resource(Health(20));
    let baseline = registry.capture(&mut world, &scene()).unwrap();
    let mut invalid = baseline.clone();
    invalid.entities[0].components[0].type_name = "missing".into();
    assert!(matches!(
        registry.prepare(&invalid),
        Err(SceneError::UnknownType { .. })
    ));
    invalid = baseline.clone();
    invalid.resources[0].type_name = "missing.resource".into();
    assert!(matches!(
        registry.prepare(&invalid),
        Err(SceneError::UnknownType { .. })
    ));
    invalid = baseline.clone();
    let repeated = invalid.entities[0].components[0].clone();
    invalid.entities[0].components.push(repeated);
    assert!(matches!(
        registry.prepare(&invalid),
        Err(SceneError::Duplicate { .. })
    ));
    invalid = baseline.clone();
    invalid.resources.push(invalid.resources[0].clone());
    assert!(matches!(
        registry.prepare(&invalid),
        Err(SceneError::Duplicate { .. })
    ));
    for fields in [
        std::collections::BTreeMap::new(),
        [("unknown".into(), SceneScalar::Unsigned("10".into()))].into(),
        [("points".into(), SceneScalar::Bool(true))].into(),
        [("points".into(), SceneScalar::Unsigned("-1".into()))].into(),
        [("points".into(), SceneScalar::Unsigned("01".into()))].into(),
        [("points".into(), SceneScalar::Float(f64::INFINITY))].into(),
    ] {
        invalid = baseline.clone();
        invalid.entities[0].components[0].fields = fields;
        assert!(matches!(
            registry.prepare(&invalid),
            Err(SceneError::Fields { .. })
        ));
    }
    assert_eq!(registry.capture(&mut world, &scene()).unwrap(), baseline);
}

struct Lossy;
impl Reflect for Lossy {
    const TYPE_NAME: &'static str = "lossy";
    const FIELDS: &'static [gridthorn_world::FieldMetadata] = Health::FIELDS;
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![ReflectValue::Unsigned(99)]
    }
}
impl SceneData for Lossy {
    fn from_scene_values(_: &[ReflectValue]) -> Result<Self, SceneValueError> {
        Ok(Self)
    }
}

#[test]
fn lossy_constructors_and_duplicate_registrations_are_rejected() {
    let mut registry = registry();
    assert!(matches!(
        registry.register_component::<Health>(),
        Err(SceneError::Reflection(_))
    ));
    assert_eq!(registry.reflection().types().count(), 2);
    registry.register_resource::<Lossy>().unwrap();
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    world.insert_resource(Lossy);
    let mut document = registry.capture(&mut world, &scene()).unwrap();
    document.resources[0]
        .fields
        .insert("points".into(), SceneScalar::Unsigned("1".into()));
    assert!(matches!(
        registry.prepare(&document),
        Err(SceneError::Construct { .. })
    ));
}
