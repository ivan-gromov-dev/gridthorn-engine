use crate::{
    GraphicsAdapterCompatibility, GraphicsAdapterKey, GraphicsBackend, WindowConfig,
    WindowedApplication,
};

#[test]
fn facade_exposes_adapter_selection_without_backend_types() {
    let key = GraphicsAdapterKey {
        backend: GraphicsBackend::Vulkan,
        vendor: 1,
        device: 2,
        name: "preference".into(),
    };
    let runtime = crate::ApplicationRuntime::new(crate::ScheduleBuilder::new().build());
    let _application =
        WindowedApplication::new(WindowConfig::default(), runtime).with_graphics_adapter(key);
    std::hint::black_box(crate::enumerate_graphics_adapters as fn() -> Vec<crate::GraphicsAdapter>);
    let runtime = crate::ApplicationRuntime::new(crate::ScheduleBuilder::new().build());
    let _application = WindowedApplication::new(WindowConfig::default(), runtime)
        .with_graphics_selection(crate::GraphicsSelection {
            device: None,
            api: Some(GraphicsBackend::Vulkan),
        });
    assert_ne!(
        GraphicsAdapterCompatibility::RequiresSurfaceValidation,
        GraphicsAdapterCompatibility::Compatible
    );
}
