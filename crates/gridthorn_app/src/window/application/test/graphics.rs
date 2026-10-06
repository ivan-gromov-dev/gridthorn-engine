use std::sync::Arc;

use gridthorn_render::{
    GraphicsAdapterCompatibility, GraphicsAdapters, RenderSurfaceError, SurfaceRenderer,
    WindowSurfaceTarget,
};
use winit::{event_loop::EventLoop, platform::windows::EventLoopBuilderExtWindows};

use super::super::WinitApplication;
use crate::{ApplicationError, WindowConfig, WindowControl, WindowLifecycle};

#[derive(Default)]
struct GraphicsProbe {
    snapshot: Option<GraphicsAdapters>,
}

impl WindowLifecycle for GraphicsProbe {
    fn graphics_adapters_initialized(&mut self, adapters: GraphicsAdapters) {
        self.snapshot = Some(adapters);
    }

    fn started(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        assert!(self.snapshot.is_some());
        control.exit();
        Ok(())
    }
}

/// Validates real surface enumeration, explicit recreation, presentation and stale selection.
#[test]
#[ignore = "requires a native Windows desktop and GPU; run this test alone"]
fn enumerates_selects_recreates_and_rejects_missing_native_adapter() {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let mut app = WinitApplication::new(WindowConfig::default(), GraphicsProbe::default());
    event_loop.run_app(&mut app).unwrap();
    assert!(app.error.is_none(), "{:?}", app.error);
    let snapshot = app.lifecycle.snapshot.take().unwrap();
    println!("graphics_adapters: {snapshot:?}");
    assert!(
        snapshot
            .adapters
            .iter()
            .any(|adapter| adapter.key == snapshot.selected
                && adapter.compatibility == GraphicsAdapterCompatibility::Compatible)
    );
    let window = Arc::clone(app.window.as_ref().unwrap());
    app.renderer.take();
    for candidate in snapshot
        .adapters
        .iter()
        .filter(|adapter| adapter.compatibility == GraphicsAdapterCompatibility::Compatible)
    {
        let mut renderer = SurfaceRenderer::with_adapter(
            WindowSurfaceTarget::new(window.clone()),
            960,
            540,
            Some(&candidate.key),
        )
        .unwrap();
        assert_eq!(renderer.graphics_adapters().selected, candidate.key);
        renderer.render().unwrap();
    }
    let mut missing = snapshot.selected;
    missing.name = "gridthorn nonexistent graphics adapter".into();
    let result =
        SurfaceRenderer::with_adapter(WindowSurfaceTarget::new(window), 960, 540, Some(&missing));
    assert!(matches!(
        result,
        Err(RenderSurfaceError::AdapterUnavailable { .. })
    ));
    app.finish(Ok(())).unwrap();
}
