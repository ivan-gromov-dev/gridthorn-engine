use super::{
    Codec,
    files::Directory,
    heap::{phase, samples, sizes, workflow},
    runtime,
};
use crate::{ScenarioState, WorldSaveCodec, WorldSaveError};
use gridthorn_simulation::GameCommandQueue;
use std::{cell::Cell, hint::black_box};

#[test]
#[ignore = "manual world-save codec/I/O/heap attribution; release, single test thread"]
fn measure_save_cold_errors_and_phases() {
    assert!(!black_box(cfg!(debug_assertions)));
    for count in sizes(&[1024, 262_144]) {
        workflow(&format!("save,vec,{count}"), || measure(count));
    }
}

fn measure(count: usize) {
    let directory = Directory::new();
    let path = directory.0.join("phase.toml");
    let mut source = runtime();
    source
        .world()
        .update_resource(|state: &mut ScenarioState<Vec<u64>, u64>| {
            state.data = (0..count as u64).collect();
            for index in 0..4096 {
                state.commands.push(index);
            }
            for index in 0..256 {
                state.random.register(&format!("stream-{index}")).unwrap();
            }
        });
    for sample in 0..samples() {
        let label = |state: &str, operation: &str| {
            format!("save,vec,{count},none,{state},{operation},{sample}")
        };
        let mut target = runtime();
        target
            .world()
            .update_resource(|state: &mut ScenarioState<Vec<u64>, u64>| {
                state.data = vec![u64::MAX; count];
            });
        let snapshot = phase(&label("isolated", "snapshot_clone"), || {
            source.snapshot().unwrap()
        });
        let payload = phase(&label("isolated", "codec_encode"), || {
            Codec
                .encode(&snapshot.state().data, &snapshot.state().commands)
                .unwrap()
        });
        let decoded = phase(&label("isolated", "codec_decode"), || {
            Codec.decode(&payload).unwrap()
        });
        assert_eq!(decoded.0, snapshot.state().data);
        let encoded = phase(&label("fresh", "save_document"), || {
            source.save_document(&Codec).unwrap()
        });
        let envelope = phase(&label("isolated", "envelope_parse"), || {
            toml::from_str::<super::super::document::Document>(&encoded).unwrap()
        });
        let serialized = phase(&label("isolated", "envelope_encode"), || {
            toml::to_string(&envelope).unwrap()
        });
        assert_eq!(serialized, encoded);
        phase(&label("isolated", "file_replace"), || {
            super::super::files::replace(&path, encoded.as_bytes()).unwrap();
        });
        phase(&label("fresh", "load_document"), || {
            target.load_document(&encoded, &Codec).unwrap();
        });
        assert_eq!(target.save_document(&Codec).unwrap(), encoded);
        let codec = TimedCodec::default();
        phase(&label("fresh", "save_file"), || {
            source.save_file(&path, &codec).unwrap();
        });
        println_codec(
            &label("inside_save_file", "codec_encode"),
            codec.encode_ns.get(),
        );
        let bytes = phase(&label("isolated", "file_read"), || {
            std::fs::read(&path).unwrap()
        });
        assert_eq!(bytes, encoded.as_bytes());
        phase(&label("fresh", "load_file"), || {
            target.load_file(&path, &codec).unwrap();
        });
        println_codec(
            &label("inside_load_file", "codec_decode"),
            codec.decode_ns.get(),
        );
        reject_paths(&mut source, &mut target, &directory, &encoded, label);
    }
}

fn reject_paths(
    source: &mut crate::ScenarioRuntime<Vec<u64>, u64>,
    target: &mut crate::ScenarioRuntime<Vec<u64>, u64>,
    directory: &Directory,
    encoded: &str,
    label: impl Fn(&str, &str) -> String,
) {
    let path = directory.0.join("phase.toml");
    let before = target.save_document(&Codec).unwrap();
    let incompatible = encoded.replacen("revision = 1", "revision = 99", 1);
    assert!(
        phase(&label("early", "reject_metadata"), || target
            .load_document(&incompatible, &Codec))
        .is_err()
    );
    assert_eq!(target.save_document(&Codec).unwrap(), before);
    let rejected = RejectedCodec;
    assert!(matches!(
        phase(&label("late", "reject_codec"), || target
            .load_document(encoded, &rejected)),
        Err(WorldSaveError::Codec(_))
    ));
    assert!(matches!(
        phase(&label("late", "reject_save_codec"), || source
            .save_file(&path, &rejected)),
        Err(WorldSaveError::Codec(_))
    ));
    assert_eq!(std::fs::read(&path).unwrap(), encoded.as_bytes());
    assert!(matches!(
        phase(&label("io_error", "missing_read"), || target
            .load_file(directory.0.join("missing.toml"), &Codec)),
        Err(WorldSaveError::Io { .. })
    ));
    let blocked = directory.0.join("destination-directory");
    std::fs::create_dir(&blocked).unwrap();
    assert!(matches!(
        phase(&label("io_error", "rename_cleanup"), || source
            .save_file(&blocked, &Codec)),
        Err(WorldSaveError::Io {
            operation: "replace",
            ..
        })
    ));
    std::fs::remove_dir(&blocked).unwrap();
    assert_eq!(std::fs::read_dir(&directory.0).unwrap().count(), 1);
    std::fs::write(&path, [0xff]).unwrap();
    assert!(matches!(
        phase(&label("early", "reject_utf8"), || target
            .load_file(&path, &Codec)),
        Err(WorldSaveError::Document(_))
    ));
    std::fs::write(&path, vec![b' '; 16 * 1024 * 1024 + 1]).unwrap();
    assert!(matches!(
        phase(&label("early", "reject_file_limit"), || target
            .load_file(&path, &Codec)),
        Err(WorldSaveError::Document(_))
    ));
    assert_eq!(target.save_document(&Codec).unwrap(), before);
    std::fs::remove_file(&path).unwrap();
}
#[derive(Default)]
struct TimedCodec {
    encode_ns: Cell<u128>,
    decode_ns: Cell<u128>,
}
impl WorldSaveCodec<Vec<u64>, u64> for TimedCodec {
    fn encode(&self, data: &Vec<u64>, commands: &GameCommandQueue<u64>) -> Result<String, String> {
        let start = std::time::Instant::now();
        let result = Codec.encode(data, commands);
        self.encode_ns.set(start.elapsed().as_nanos());
        result
    }
    fn decode(&self, payload: &str) -> Result<(Vec<u64>, GameCommandQueue<u64>), String> {
        let start = std::time::Instant::now();
        let result = Codec.decode(payload);
        self.decode_ns.set(start.elapsed().as_nanos());
        result
    }
}
struct RejectedCodec;
impl WorldSaveCodec<Vec<u64>, u64> for RejectedCodec {
    fn encode(&self, _: &Vec<u64>, _: &GameCommandQueue<u64>) -> Result<String, String> {
        Err("intentional codec failure".into())
    }
    fn decode(&self, _: &str) -> Result<(Vec<u64>, GameCommandQueue<u64>), String> {
        Err("intentional codec failure".into())
    }
}
fn println_codec(label: &str, elapsed: u128) {
    if std::env::var_os("GRIDTHORN_IO_HEAP").is_none() {
        println!("io_phase,{label},{elapsed},cpu,0,0,0,0");
    }
}
