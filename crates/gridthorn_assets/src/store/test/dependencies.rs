use super::{AssetId, AssetStore, AssetStoreError, Fixture, id};

#[test]
fn identities_are_stable_and_reject_ambiguous_paths() {
    for path in [
        "", "/asset", "a//b", "../a", "a/../b", "./a", "a\\b", "C:/a", "a/", "a\0b", "a.", "a ",
    ] {
        assert!(AssetId::new(path).is_err(), "{path:?}");
    }
    assert_eq!(id("images/player.png").as_str(), "images/player.png");
    assert_eq!(id("images/player.png"), id("images/player.png"));
}

#[test]
fn propagates_transitive_diamond_dependencies_once_in_stable_order() {
    let fixture = Fixture::new();
    let mut store = AssetStore::new(&fixture.0);
    for name in ["z", "c", "b", "a", "unrelated"] {
        fixture.write(name, b"initial");
        store.load_source(id(name)).unwrap();
    }
    store
        .set_dependencies(&id("a"), &[id("b"), id("c"), id("c")])
        .unwrap();
    store.set_dependencies(&id("b"), &[id("z")]).unwrap();
    store.set_dependencies(&id("c"), &[id("z")]).unwrap();
    fixture.write("z", b"changed");
    assert_eq!(
        store.reload_changed().unwrap(),
        vec![id("z"), id("b"), id("c"), id("a")]
    );
    assert!(store.reload_changed().unwrap().is_empty());
}

#[test]
fn rejects_cycles_and_missing_ids_without_changing_edges() {
    let fixture = Fixture::new();
    let mut store = AssetStore::new(&fixture.0);
    for name in ["a", "b", "c"] {
        fixture.write(name, b"old");
        store.load_source(id(name)).unwrap();
    }
    store.set_dependencies(&id("a"), &[id("b")]).unwrap();
    store.set_dependencies(&id("b"), &[id("c")]).unwrap();
    assert!(matches!(
        store.set_dependencies(&id("c"), &[id("a")]),
        Err(AssetStoreError::DependencyCycle { .. })
    ));
    assert!(matches!(
        store.set_dependencies(&id("a"), &[id("a")]),
        Err(AssetStoreError::DependencyCycle { .. })
    ));
    assert!(matches!(
        store.set_dependencies(&id("a"), &[id("missing")]),
        Err(AssetStoreError::Unknown { .. })
    ));
    assert!(store.set_dependencies(&id("missing"), &[]).is_err());
    fixture.write("c", b"new");
    assert_eq!(
        store.reload_changed().unwrap(),
        vec![id("c"), id("b"), id("a")]
    );
    store.set_dependencies(&id("a"), &[]).unwrap();
    fixture.write("c", b"newer");
    assert_eq!(store.reload_changed().unwrap(), vec![id("c"), id("b")]);
}
