use super::super::{RuntimeWindowLifecycle, WindowLifecycle};
use crate::ApplicationRuntime;
use gridthorn_render::{GraphicsAdapterKey, GraphicsAdapters, GraphicsBackend};

#[test]
fn publishes_effective_adapter_as_a_runtime_resource() {
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(
        gridthorn_world::ScheduleBuilder::new().build(),
    ));
    let snapshot = GraphicsAdapters {
        adapters: Vec::new(),
        selected: GraphicsAdapterKey {
            backend: GraphicsBackend::Vulkan,
            vendor: 1,
            device: 2,
            name: "selected".into(),
        },
    };
    assert!(
        lifecycle
            .runtime
            .world()
            .read_resource(|_: &GraphicsAdapters| ())
            .is_none()
    );
    lifecycle.graphics_adapters_initialized(snapshot.clone());
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|adapters: &GraphicsAdapters| adapters.clone()),
        Some(snapshot)
    );
}
