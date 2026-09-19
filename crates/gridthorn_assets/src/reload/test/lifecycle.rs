use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use super::{AssetReloadError, AssetReloader, Fixture, finish, id};

#[test]
fn shutdown_joins_pending_work_and_discards_unapplied_results() {
    let fixture = Fixture::new();
    let finished = Arc::new(AtomicBool::new(false));
    let worker_finished = Arc::clone(&finished);
    let mut reloader = AssetReloader::start(fixture.store(), move |assets| {
        let result = assets.reload_changed();
        worker_finished.store(true, Ordering::SeqCst);
        result
    })
    .unwrap();
    fixture.write("sprite.ppm", b"P3\n1 1\n255\n0 0 255\n");
    reloader.request_reload().unwrap();
    reloader.shutdown().unwrap();
    assert!(finished.load(Ordering::SeqCst));
    assert!(!reloader.is_pending());
    assert_eq!(
        reloader
            .assets()
            .texture(&id("sprite.ppm"))
            .unwrap()
            .rgba8(),
        [255, 0, 0, 255]
    );
    assert!(matches!(reloader.poll(), Err(AssetReloadError::Stopped)));
    assert!(matches!(
        reloader.request_reload(),
        Err(AssetReloadError::Stopped)
    ));
    reloader.shutdown().unwrap();
}

#[test]
fn drop_joins_worker_and_idle_shutdown_needs_no_request() {
    let fixture = Fixture::new();
    let finished = Arc::new(AtomicBool::new(false));
    let worker_finished = Arc::clone(&finished);
    let mut reloader = AssetReloader::start(fixture.store(), move |assets| {
        let result = assets.reload_changed();
        worker_finished.store(true, Ordering::SeqCst);
        result
    })
    .unwrap();
    reloader.request_reload().unwrap();
    drop(reloader);
    assert!(finished.load(Ordering::SeqCst));
    let mut idle = AssetReloader::new(fixture.store()).unwrap();
    assert!(idle.poll().unwrap().is_none());
    idle.shutdown().unwrap();
}

#[test]
fn worker_panic_returns_a_typed_failure_and_keeps_visible_data() {
    let fixture = Fixture::new();
    let mut reloader =
        AssetReloader::start(fixture.store(), |_| panic!("injected worker failure")).unwrap();
    reloader.request_reload().unwrap();
    assert!(matches!(
        finish(&mut reloader),
        Err(AssetReloadError::Stopped)
    ));
    assert!(!reloader.is_pending());
    assert!(matches!(
        reloader.request_reload(),
        Err(AssetReloadError::Stopped)
    ));
    assert!(reloader.assets().texture(&id("sprite.ppm")).is_some());
    assert!(matches!(
        reloader.shutdown(),
        Err(AssetReloadError::Stopped)
    ));
}

#[test]
fn service_can_be_owned_by_a_world_resource() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AssetReloader>();
}
