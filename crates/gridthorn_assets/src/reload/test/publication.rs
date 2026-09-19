use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
    mpsc,
};
use std::time::Duration;

use super::{AssetReloadError, AssetReloader, Fixture, finish, id};

#[test]
fn blocked_worker_does_not_block_frames_or_queue_duplicate_requests() {
    let fixture = Fixture::new();
    let (release, gate) = mpsc::channel();
    let (started, ready) = mpsc::channel();
    let calls = Arc::new(AtomicUsize::new(0));
    let worker_calls = Arc::clone(&calls);
    let mut reloader = AssetReloader::start(fixture.store(), move |assets| {
        worker_calls.fetch_add(1, Ordering::SeqCst);
        started.send(()).unwrap();
        gate.recv_timeout(Duration::from_secs(5)).unwrap();
        assets.reload_changed()
    })
    .unwrap();
    let old = reloader
        .assets()
        .texture(&id("sprite.ppm"))
        .unwrap()
        .clone();
    fixture.write("sprite.ppm", b"P3\n1 1\n255\n0 255 0\n");
    assert!(reloader.request_reload().unwrap());
    ready.recv_timeout(Duration::from_secs(5)).unwrap();
    for _ in 0..100 {
        assert!(!reloader.request_reload().unwrap());
        assert!(reloader.poll().unwrap().is_none());
        assert!(old.shares_data_with(reloader.assets().texture(&id("sprite.ppm")).unwrap()));
    }
    release.send(()).unwrap();
    assert_eq!(
        finish(&mut reloader).unwrap(),
        vec![id("sprite.ppm"), id("scene.txt")]
    );
    assert_eq!(
        reloader
            .assets()
            .texture(&id("sprite.ppm"))
            .unwrap()
            .rgba8(),
        [0, 255, 0, 255]
    );
    assert_eq!(old.rgba8(), [255, 0, 0, 255]);
    assert!(!reloader.is_pending());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    reloader.shutdown().unwrap();
}

#[test]
fn completed_batches_are_invisible_until_polled_and_later_edits_need_new_requests() {
    let fixture = Fixture::new();
    let (prepared, ready) = mpsc::channel();
    let mut reloader = AssetReloader::start(fixture.store(), move |assets| {
        let result = assets.reload_changed();
        prepared.send(()).unwrap();
        result
    })
    .unwrap();
    fixture.write("sprite.ppm", b"P3\n1 1\n255\n0 255 0\n");
    reloader.request_reload().unwrap();
    ready.recv_timeout(Duration::from_secs(5)).unwrap();
    fixture.write("sprite.ppm", b"P3\n1 1\n255\n0 0 255\n");
    assert!(reloader.is_pending());
    assert!(!reloader.request_reload().unwrap());
    assert_eq!(
        reloader
            .assets()
            .texture(&id("sprite.ppm"))
            .unwrap()
            .rgba8(),
        [255, 0, 0, 255]
    );
    finish(&mut reloader).unwrap();
    assert_eq!(
        reloader
            .assets()
            .texture(&id("sprite.ppm"))
            .unwrap()
            .rgba8(),
        [0, 255, 0, 255]
    );
    reloader.request_reload().unwrap();
    finish(&mut reloader).unwrap();
    assert_eq!(
        reloader
            .assets()
            .texture(&id("sprite.ppm"))
            .unwrap()
            .rgba8(),
        [0, 0, 255, 255]
    );
    let unchanged = reloader
        .assets()
        .texture(&id("sprite.ppm"))
        .unwrap()
        .clone();
    reloader.request_reload().unwrap();
    assert!(finish(&mut reloader).unwrap().is_empty());
    assert!(unchanged.shares_data_with(reloader.assets().texture(&id("sprite.ppm")).unwrap()));
}

#[test]
fn failed_preparation_preserves_every_asset_and_retry_uses_last_good_baseline() {
    let fixture = Fixture::new();
    let mut reloader = AssetReloader::new(fixture.store()).unwrap();
    fixture.write("scene.txt", b"new metadata");
    fixture.write("sprite.ppm", b"unfinished image");
    reloader.request_reload().unwrap();
    let error = finish(&mut reloader).unwrap_err();
    assert!(matches!(error, AssetReloadError::Prepare { .. }));
    assert!(error.to_string().contains("sprite.ppm"));
    assert!(!reloader.is_pending());
    assert_eq!(
        &*reloader.assets().source(&id("scene.txt")).unwrap(),
        b"sprite.ppm"
    );
    fixture.write("sprite.ppm", b"P3\n1 1\n255\n0 0 255\n");
    reloader.request_reload().unwrap();
    assert_eq!(
        finish(&mut reloader).unwrap(),
        vec![id("sprite.ppm"), id("scene.txt")]
    );
    assert_eq!(
        &*reloader.assets().source(&id("scene.txt")).unwrap(),
        b"new metadata"
    );
}
