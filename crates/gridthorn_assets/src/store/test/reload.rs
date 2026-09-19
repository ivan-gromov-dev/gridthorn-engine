use super::{AssetStore, AssetStoreError, Fixture, id};

#[test]
fn same_length_edits_replace_current_texture_but_preserve_old_snapshots() {
    let fixture = Fixture::new();
    fixture.texture("sprite.ppm", [255, 0, 0]);
    let mut store = AssetStore::new(&fixture.0);
    store.load_texture(id("sprite.ppm")).unwrap();
    let old = store.texture(&id("sprite.ppm")).unwrap().clone();
    assert!(store.reload_changed().unwrap().is_empty());
    assert!(old.shares_data_with(store.texture(&id("sprite.ppm")).unwrap()));
    fixture.texture("sprite.ppm", [0, 255, 0]);
    assert_eq!(store.reload_changed().unwrap(), vec![id("sprite.ppm")]);
    let new = store.texture(&id("sprite.ppm")).unwrap();
    assert_eq!(new.rgba8(), [0, 255, 0, 255]);
    assert_eq!(old.rgba8(), [255, 0, 0, 255]);
    assert!(!new.shares_data_with(&old));
}

#[test]
fn failed_batch_preserves_all_assets_and_retries_after_repair() {
    let fixture = Fixture::new();
    let mut store = AssetStore::new(&fixture.0);
    for name in ["a.ppm", "b.ppm"] {
        fixture.texture(name, [255, 0, 0]);
        store.load_texture(id(name)).unwrap();
    }
    let old_bytes = store.source(&id("a.ppm")).unwrap();
    let old_texture = store.texture(&id("a.ppm")).unwrap().clone();
    fixture.texture("a.ppm", [0, 255, 0]);
    fixture.write("b.ppm", b"incomplete image");
    let error = store.reload_changed().unwrap_err();
    assert!(matches!(error, AssetStoreError::Texture { .. }));
    assert!(error.to_string().contains("b.ppm"));
    assert_eq!(store.source(&id("a.ppm")).unwrap(), old_bytes);
    assert!(old_texture.shares_data_with(store.texture(&id("a.ppm")).unwrap()));
    fixture.texture("b.ppm", [0, 0, 255]);
    assert_eq!(
        store.reload_changed().unwrap(),
        vec![id("a.ppm"), id("b.ppm")]
    );
    assert_eq!(
        store.texture(&id("a.ppm")).unwrap().rgba8(),
        [0, 255, 0, 255]
    );
}

#[test]
fn deleted_sources_preserve_state_and_recover_after_recreation() {
    let fixture = Fixture::new();
    fixture.write("metadata", b"old");
    let mut store = AssetStore::new(&fixture.0);
    store.load_source(id("metadata")).unwrap();
    std::fs::remove_file(fixture.0.join("metadata")).unwrap();
    assert!(matches!(
        store.reload_changed(),
        Err(AssetStoreError::Read { .. })
    ));
    assert_eq!(&*store.source(&id("metadata")).unwrap(), b"old");
    fixture.write("metadata", b"new");
    assert_eq!(store.reload_changed().unwrap(), vec![id("metadata")]);
}

#[test]
fn source_changes_invalidate_textures_and_keep_unrelated_allocations() {
    let fixture = Fixture::new();
    fixture.write("metadata", b"old");
    fixture.texture("sprite.ppm", [255, 0, 0]);
    fixture.texture("other.ppm", [0, 0, 255]);
    let mut store = AssetStore::new(&fixture.0);
    store.load_source(id("metadata")).unwrap();
    store.load_texture(id("sprite.ppm")).unwrap();
    store.load_texture(id("other.ppm")).unwrap();
    store
        .set_dependencies(&id("sprite.ppm"), &[id("metadata")])
        .unwrap();
    let old = store.texture(&id("sprite.ppm")).unwrap().clone();
    let unrelated = store.texture(&id("other.ppm")).unwrap().clone();
    fixture.write("metadata", b"new");
    assert_eq!(
        store.reload_changed().unwrap(),
        vec![id("metadata"), id("sprite.ppm")]
    );
    assert!(!old.shares_data_with(store.texture(&id("sprite.ppm")).unwrap()));
    assert!(unrelated.shares_data_with(store.texture(&id("other.ppm")).unwrap()));
}

#[test]
fn failed_registration_leaves_id_available_and_duplicates_preserve_kind() {
    let fixture = Fixture::new();
    let mut store = AssetStore::new(&fixture.0);
    assert!(store.load_texture(id("sprite.ppm")).is_err());
    fixture.write("sprite.ppm", b"invalid");
    assert!(store.load_texture(id("sprite.ppm")).is_err());
    assert!(store.source(&id("sprite.ppm")).is_none());
    fixture.texture("sprite.ppm", [255, 0, 0]);
    store.load_texture(id("sprite.ppm")).unwrap();
    assert!(matches!(
        store.load_source(id("sprite.ppm")),
        Err(AssetStoreError::Duplicate { .. })
    ));
    assert!(store.texture(&id("sprite.ppm")).is_some());
}

#[test]
fn store_and_snapshots_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AssetStore>();
    assert_send_sync::<crate::TextureAsset>();
}
