use gridthorn_world::ScheduleBuilder;

use crate::{SceneDocument, SceneError, SceneMigrations, SceneValueError};

use super::{Health, registry, scene};

fn rename_points(mut document: SceneDocument) -> Result<SceneDocument, SceneValueError> {
    for entity in &mut document.entities {
        for record in &mut entity.components {
            let points = record
                .fields
                .remove("old_points")
                .ok_or_else(|| SceneValueError("old_points missing".into()))?;
            record.fields.insert("points".into(), points);
        }
    }
    document.schema_version = 1;
    Ok(document)
}

#[test]
fn migration_is_explicit_deterministic_and_validated_before_commit() {
    let registry = registry();
    let mut runtime = ScheduleBuilder::new().build();
    let mut world = runtime.world();
    world.spawn_in_scene(scene(), Health(42));
    let mut document = registry.capture(&mut world, &scene()).unwrap();
    document.schema_version = 0;
    let record = &mut document.entities[0].components[0];
    let points = record.fields.remove("points").unwrap();
    record.fields.insert("old_points".into(), points);
    assert!(matches!(
        SceneMigrations::default().upgrade(document.clone()),
        Err(SceneError::Schema(0))
    ));
    let mut migrations = SceneMigrations::default();
    migrations.register(0, rename_points).unwrap();
    let upgraded = migrations.upgrade(document.clone()).unwrap();
    assert_eq!(upgraded, migrations.upgrade(document).unwrap());
    assert_eq!(upgraded.schema_version, 1);
    let loaded = registry.prepare(&upgraded).unwrap().commit(&mut world);
    assert_eq!(
        world.read_component(loaded.entities[0], |value: &Health| value.0),
        Some(42)
    );
}

#[test]
fn migration_registration_future_schemas_and_bad_callbacks_fail() {
    let mut migrations = SceneMigrations::default();
    assert!(matches!(
        migrations.register(1, rename_points),
        Err(SceneError::MigrationRegistration(1))
    ));
    migrations.register(0, Ok).unwrap();
    assert!(matches!(
        migrations.register(0, rename_points),
        Err(SceneError::MigrationRegistration(0))
    ));
    let mut document = SceneDocument::new(&scene());
    assert_eq!(migrations.upgrade(document.clone()).unwrap(), document);
    document.schema_version = 2;
    assert!(matches!(
        migrations.upgrade(document.clone()),
        Err(SceneError::Schema(2))
    ));
    document.schema_version = 0;
    assert!(matches!(
        migrations.upgrade(document.clone()),
        Err(SceneError::Migration { .. })
    ));
    let mut migrations = SceneMigrations::default();
    migrations
        .register(0, |_| Err(SceneValueError("bad legacy fields".into())))
        .unwrap();
    assert!(matches!(
        migrations.upgrade(document),
        Err(SceneError::Migration { .. })
    ));
}
