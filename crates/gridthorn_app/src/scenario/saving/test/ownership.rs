use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use super::*;

struct Root {
    value: u64,
    clones: Arc<AtomicUsize>,
}

impl Clone for Root {
    fn clone(&self) -> Self {
        self.clones.fetch_add(1, Ordering::Relaxed);
        Self {
            value: self.value,
            clones: Arc::clone(&self.clones),
        }
    }
}

struct RootCodec(Arc<AtomicUsize>);

impl WorldSaveCodec<Root, u64> for RootCodec {
    fn encode(&self, data: &Root, _: &GameCommandQueue<u64>) -> Result<String, String> {
        Ok(data.value.to_string())
    }

    fn decode(&self, payload: &str) -> Result<(Root, GameCommandQueue<u64>), String> {
        Ok((
            Root {
                value: payload.parse().map_err(|_| "invalid root".to_owned())?,
                clones: Arc::clone(&self.0),
            },
            GameCommandQueue::new(),
        ))
    }
}

#[test]
fn document_load_moves_decoded_root_without_cloning_live_or_decoded_data() {
    let clones = Arc::new(AtomicUsize::new(0));
    let scenario = Scenario::new(
        "ownership",
        1,
        ScenarioState {
            data: Root {
                value: 42,
                clones: Arc::clone(&clones),
            },
            commands: GameCommandQueue::new(),
            random: RandomStreams::new(7),
        },
    )
    .unwrap();
    let mut runtime = ScenarioRuntime::new(
        ScheduleBuilder::new().build(),
        FixedStepConfig::default(),
        scenario,
    )
    .unwrap();
    let codec = RootCodec(Arc::clone(&clones));
    let document = runtime.save_document(&codec).unwrap();
    runtime
        .world()
        .update_resource(|state: &mut ScenarioState<Root, u64>| state.data.value = 99);
    clones.store(0, Ordering::Relaxed);
    runtime.load_document(&document, &codec).unwrap();
    assert_eq!(clones.load(Ordering::Relaxed), 0);
    assert_eq!(
        runtime
            .world()
            .read_resource(|state: &ScenarioState<Root, u64>| state.data.value),
        Some(42)
    );
    let incompatible = document.replace("revision = 1", "revision = 2");
    assert!(runtime.load_document(&incompatible, &codec).is_err());
    assert_eq!(clones.load(Ordering::Relaxed), 0);
    assert_eq!(
        runtime
            .world()
            .read_resource(|state: &ScenarioState<Root, u64>| state.data.value),
        Some(42)
    );
}
