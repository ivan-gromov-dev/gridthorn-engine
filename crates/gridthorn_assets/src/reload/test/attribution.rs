use super::super::test::{
    Fixture,
    heap::{phase, samples, sizes, workflow},
    id,
};
use crate::{AssetReloadError, AssetReloader, AssetStore, AssetStoreError, TextureAsset};
use image::ImageEncoder;
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

#[test]
#[ignore = "manual asset attribution; release, single test thread"]
fn measure_asset_cold_branching_errors() {
    assert!(!black_box(cfg!(debug_assertions)));
    for kind in ["raw", "ppm", "png"] {
        if std::env::var("GRIDTHORN_IO_KIND").is_ok_and(|value| value != kind) {
            continue;
        }
        for count in sizes(if kind == "raw" {
            &[64, 512][..]
        } else {
            &[16][..]
        }) {
            for topology in ["diamond", "fanout", "disconnected"] {
                if std::env::var("GRIDTHORN_IO_GRAPH").is_ok_and(|value| value != topology) {
                    continue;
                }
                workflow(&format!("asset,{kind},{count},{topology}"), || {
                    measure(kind, count, topology);
                });
            }
        }
    }
}

fn measure(kind: &str, count: usize, topology: &str) {
    assert!(count >= 4);
    let fixture = Fixture::new();
    let ids = (0..count)
        .map(|i| id(&format!("asset-{i:04}.{kind}")))
        .collect::<Vec<_>>();
    let edges = graph(count, topology);
    let initial = payload(kind, 0);
    let changed = payload(kind, 1);
    for asset in &ids {
        fixture.write(asset.as_str(), &initial);
    }
    let label = |state: &str, operation: &str, sample: usize| {
        format!("asset,{kind},{count},{topology},{state},{operation},{sample}")
    };
    for sample in 0..samples() {
        let mut store = phase(&label("fresh", "register", sample), || {
            let mut store = AssetStore::new(&fixture.0);
            for asset in &ids {
                if kind == "raw" {
                    store.load_source(asset.clone()).unwrap();
                } else {
                    store.load_texture(asset.clone()).unwrap();
                }
            }
            store
        });
        phase(&label("fresh", "graph_registration", sample), || {
            for (index, dependencies) in edges.iter().enumerate() {
                let dependencies = dependencies
                    .iter()
                    .map(|index| ids[*index].clone())
                    .collect::<Vec<_>>();
                store.set_dependencies(&ids[index], &dependencies).unwrap();
            }
        });
        let buffers = phase(&label("isolated", "read_all", sample), || {
            ids.iter()
                .map(|asset| std::fs::read(fixture.0.join(asset.as_str())).unwrap())
                .collect::<Vec<_>>()
        });
        if kind != "raw" {
            let textures = phase(&label("isolated", "decode_all", sample), || {
                buffers
                    .iter()
                    .map(|bytes| {
                        TextureAsset::decode(bytes, &fixture.0.join(format!("image.{kind}")))
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
            });
            assert!(
                textures
                    .iter()
                    .all(|texture| texture.dimensions() == [256, 256])
            );
            drop(textures);
        }
        drop(buffers);
        let ordered = expected(&ids, topology);
        let mut worker = AssetReloader::new(store.snapshot()).unwrap();
        let before = store.source(&ids[count - 1]).unwrap();
        fixture.write(ids[count - 1].as_str(), &changed);
        assert_eq!(
            phase(&label("changed", "sync_scan", sample), || store
                .reload_changed()
                .unwrap()),
            ordered
        );
        let reply = phase(&label("changed", "worker_prepare", sample), || {
            wait_ready(&mut worker, false)
        });
        let published = phase(&label("changed", "publish", sample), || {
            worker.publish(reply).unwrap().unwrap()
        });
        assert_eq!(published, ordered);
        assert_eq!(before.as_ref(), initial);
        reject_and_recover(
            &mut store,
            &mut worker,
            &fixture,
            &ids,
            &initial,
            &changed,
            |state, operation| label(state, operation, sample),
        );
        worker.shutdown().unwrap();
        for asset in &ids {
            fixture.write(asset.as_str(), &initial);
        }
    }
}

fn reject_and_recover(
    store: &mut AssetStore,
    worker: &mut AssetReloader,
    fixture: &Fixture,
    ids: &[crate::AssetId],
    initial: &[u8],
    changed: &[u8],
    label: impl Fn(&str, &str) -> String,
) {
    let committed = store.source(&ids[ids.len() - 1]).unwrap();
    fixture.write(ids[0].as_str(), changed);
    std::fs::remove_file(fixture.0.join(ids[ids.len() - 1].as_str())).unwrap();
    assert!(matches!(
        phase(&label("missing", "sync_reject"), || store.reload_changed()),
        Err(AssetStoreError::Read { .. })
    ));
    let reply = phase(&label("missing", "worker_prepare"), || {
        wait_ready(worker, true)
    });
    assert!(matches!(
        phase(&label("missing", "publish_reject"), || worker
            .publish(reply)),
        Err(AssetReloadError::Prepare { .. })
    ));
    assert_eq!(store.source(&ids[0]).unwrap().as_ref(), initial);
    assert_eq!(store.source(&ids[ids.len() - 1]).unwrap(), committed);
    fixture.write(ids[ids.len() - 1].as_str(), changed);
    if store.texture(&ids[ids.len() - 1]).is_some() {
        fixture.write(ids[ids.len() - 1].as_str(), b"not an image");
        assert!(matches!(
            phase(&label("invalid", "decode_reject"), || store
                .reload_changed()),
            Err(AssetStoreError::Texture { .. })
        ));
        assert_eq!(store.source(&ids[0]).unwrap().as_ref(), initial);
        fixture.write(ids[ids.len() - 1].as_str(), changed);
    }
    phase(&label("recovery", "sync_scan"), || {
        store.reload_changed().unwrap()
    });
    let reply = phase(&label("recovery", "worker_prepare"), || {
        wait_ready(worker, false)
    });
    phase(&label("recovery", "publish"), || {
        worker.publish(reply).unwrap().unwrap()
    });
    assert_eq!(worker.assets().source(&ids[0]).unwrap().as_ref(), changed);
}
fn wait_ready(worker: &mut AssetReloader, rejection: bool) -> super::super::worker::Reply {
    assert!(worker.request_reload().unwrap());
    assert!(!worker.request_reload().unwrap());
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        match worker.replies.lock().unwrap().try_recv() {
            Ok(reply) => {
                assert_eq!(reply.is_err(), rejection);
                return reply;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
            Err(error) => panic!("worker failed: {error}"),
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn graph(count: usize, topology: &str) -> Vec<Vec<usize>> {
    (0..count)
        .map(|i| match topology {
            "diamond" => [i + 1, i + 2].into_iter().filter(|j| *j < count).collect(),
            "fanout" => {
                if i == count - 1 {
                    vec![]
                } else {
                    vec![count - 1]
                }
            }
            "disconnected" => {
                let end = ((i / 32 + 1) * 32).min(count) - 1;
                if i == end { vec![] } else { vec![end] }
            }
            _ => unreachable!(),
        })
        .collect()
}

fn expected(ids: &[crate::AssetId], topology: &str) -> Vec<crate::AssetId> {
    if topology == "diamond" {
        return ids.iter().rev().cloned().collect();
    }
    let start = if topology == "disconnected" {
        (ids.len() - 1) / 32 * 32
    } else {
        0
    };
    std::iter::once(ids[ids.len() - 1].clone())
        .chain(ids[start..ids.len() - 1].iter().cloned())
        .collect()
}

fn payload(kind: &str, value: u8) -> Vec<u8> {
    match kind {
        "raw" => vec![value; 65536],
        "ppm" => {
            let mut bytes = b"P6\n256 256\n255\n".to_vec();
            bytes.extend(vec![value; 256 * 256 * 3]);
            bytes
        }
        "png" => {
            let mut bytes = Vec::new();
            image::codecs::png::PngEncoder::new(&mut bytes)
                .write_image(
                    &vec![value; 256 * 256 * 4],
                    256,
                    256,
                    image::ExtendedColorType::Rgba8,
                )
                .unwrap();
            bytes
        }
        _ => unreachable!(),
    }
}
