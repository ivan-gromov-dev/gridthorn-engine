use super::{
    Health,
    heap::{phase, samples, sizes, workflow},
    registry, scene,
};
use crate::{SceneData, SceneDocument, SceneError, SceneScalar, SceneValueError};
use gridthorn_world::{FieldMetadata, Reflect, ReflectValue, ScheduleBuilder, ValueKind};
use std::{
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
};

struct Label(String);
impl Reflect for Label {
    const TYPE_NAME: &'static str = "game.label";
    const FIELDS: &'static [FieldMetadata] = &[FieldMetadata {
        name: "text",
        kind: ValueKind::Text,
    }];
    fn reflect_values(&self) -> Vec<ReflectValue> {
        vec![ReflectValue::Text(self.0.clone())]
    }
}
impl SceneData for Label {
    fn from_scene_values(values: &[ReflectValue]) -> Result<Self, SceneValueError> {
        match values {
            [ReflectValue::Text(text)] if !text.is_empty() => Ok(Self(text.clone())),
            _ => Err(SceneValueError("label must have text".into())),
        }
    }
}
static NEXT: AtomicU64 = AtomicU64::new(0);

#[test]
#[ignore = "manual scene phase/heap attribution; release, single test thread"]
fn measure_scene_cold_errors_and_phases() {
    assert!(!black_box(cfg!(debug_assertions)));
    for count in sizes(&[1000, 10_000]) {
        workflow(&format!("scene,mixed,{count}"), || measure(count));
    }
}

fn measure(count: usize) {
    for sample in 0..samples() {
        let label = |state: &str, operation: &str| {
            format!("scene,mixed,{count},none,{state},{operation},{sample}")
        };
        let mut registry = registry();
        registry.register_component::<Label>().unwrap();
        let mut runtime = ScheduleBuilder::new().build();
        let mut world = runtime.world();
        for index in 0..count {
            let entity = world.spawn_in_scene(scene(), Health(index as u64 + 1));
            world.insert_component(
                entity,
                Label(format!("{index} 日本語 Привет العربية e\u{301}")),
            );
        }
        let persistent = world.spawn(Health(99));
        world.insert_resource(Health(17));
        let document = phase(&label("fresh", "capture"), || {
            registry.capture(&mut world, &scene()).unwrap()
        });
        let encoded = phase(&label("isolated", "serializer"), || {
            toml::to_string_pretty(&document).unwrap()
        });
        assert_eq!(
            encoded,
            phase(&label("fresh", "encode_validated"), || document
                .to_toml()
                .unwrap())
        );
        let path = std::env::temp_dir().join(format!(
            "gridthorn-scene-phase-{}-{}.toml",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        phase(&label("caller_io", "write"), || {
            std::fs::write(&path, &encoded).unwrap();
        });
        let input = phase(&label("caller_io", "read"), || {
            std::fs::read_to_string(&path).unwrap()
        });
        let parsed = phase(&label("fresh", "parse"), || {
            SceneDocument::from_toml(&input).unwrap()
        });
        assert_eq!(parsed, document);
        let prepared = phase(&label("fresh", "prepare"), || {
            registry.prepare(&parsed).unwrap()
        });
        let loaded = phase(&label("fresh", "commit"), || prepared.commit(&mut world));
        assert_eq!(loaded.entities.len(), count);
        assert_eq!(
            world.read_component(persistent, |health: &Health| health.0),
            Some(99)
        );
        assert_eq!(world.read_resource(|health: &Health| health.0), Some(17));
        let before = registry.capture(&mut world, &scene()).unwrap();
        let mut invalid = before.clone();
        invalid.engine = "=99.0.0".into();
        assert!(matches!(
            phase(&label("early", "reject_engine"), || registry
                .prepare(&invalid)),
            Err(SceneError::Engine(_))
        ));
        invalid = before.clone();
        invalid
            .entities
            .last_mut()
            .unwrap()
            .components
            .iter_mut()
            .find(|record| record.type_name == "game.health")
            .unwrap()
            .fields
            .insert("points".into(), SceneScalar::Unsigned("0".into()));
        assert!(matches!(
            phase(&label("late", "reject_constructor"), || registry
                .prepare(&invalid)),
            Err(SceneError::Construct { .. })
        ));
        invalid = before.clone();
        invalid.entities.last_mut().unwrap().components[0].type_name = "unknown.type".into();
        assert!(matches!(
            phase(&label("late", "reject_type"), || registry.prepare(&invalid)),
            Err(SceneError::UnknownType { .. })
        ));
        let malformed = format!("{}\n[", &encoded[..encoded.len() - 1]);
        assert!(matches!(
            phase(&label("late", "reject_parse"), || SceneDocument::from_toml(
                &malformed
            )),
            Err(SceneError::Decode(_))
        ));
        assert_eq!(registry.capture(&mut world, &scene()).unwrap(), before);
        std::fs::remove_file(path).unwrap();
    }
}
