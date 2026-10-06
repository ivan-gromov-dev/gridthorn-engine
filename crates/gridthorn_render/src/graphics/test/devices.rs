use super::super::{
    GraphicsAdapter, GraphicsAdapterCompatibility, GraphicsAdapterKey, GraphicsBackend,
    GraphicsDeviceKey, GraphicsSelection, graphics_devices, resolve,
};
use crate::RenderSurfaceError;

fn record(api: GraphicsBackend) -> GraphicsAdapter {
    GraphicsAdapter {
        key: GraphicsAdapterKey {
            backend: api,
            vendor: 4318,
            device: 9352,
            name: "NVIDIA GeForce RTX 3070".into(),
        },
        software: false,
        compatibility: GraphicsAdapterCompatibility::Compatible,
    }
}

#[test]
fn groups_a_card_across_apis_and_keeps_software_separate() {
    let vulkan = record(GraphicsBackend::Vulkan);
    let dx = record(GraphicsBackend::Direct3D12);
    let mut gl = record(GraphicsBackend::OpenGl);
    gl.key.device = 0;
    gl.key.name.push_str("/PCIe/SSE2");
    let mut software = dx.clone();
    software.key.vendor = 5140;
    software.key.device = 140;
    software.key.name = "Microsoft Basic Render Driver".into();
    software.software = true;
    let records = [vulkan, dx.clone(), software, gl];
    let devices = graphics_devices(&records);
    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].apis.len(), 3);
    let selection = GraphicsSelection {
        device: Some(devices[0].key.clone()),
        api: Some(GraphicsBackend::Direct3D12),
    };
    assert_eq!(resolve(&records, &selection).unwrap(), Some(dx.key));
}

#[test]
fn supports_independent_automatic_device_and_api_preferences() {
    let records = [
        record(GraphicsBackend::Vulkan),
        record(GraphicsBackend::Direct3D12),
    ];
    assert_eq!(
        resolve(&records, &GraphicsSelection::default()).unwrap(),
        None
    );
    let mut selection = GraphicsSelection {
        device: None,
        api: Some(GraphicsBackend::Direct3D12),
    };
    assert_eq!(
        resolve(&records, &selection).unwrap(),
        Some(records[1].key.clone())
    );
    selection.device = Some(graphics_devices(&records)[0].key.clone());
    selection.api = None;
    assert_eq!(
        resolve(&records, &selection).unwrap(),
        Some(records[0].key.clone())
    );
}

#[test]
fn rejects_missing_api_and_indistinguishable_cards() {
    let record = record(GraphicsBackend::Vulkan);
    let device = graphics_devices(std::slice::from_ref(&record))[0]
        .key
        .clone();
    let mut selection = GraphicsSelection {
        device: Some(device),
        api: Some(GraphicsBackend::Metal),
    };
    assert!(matches!(
        resolve(std::slice::from_ref(&record), &selection),
        Err(RenderSurfaceError::GraphicsSelectionUnavailable { .. })
    ));
    selection.api = Some(GraphicsBackend::Vulkan);
    assert!(matches!(
        resolve(&[record.clone(), record], &selection),
        Err(RenderSurfaceError::GraphicsSelectionAmbiguous { .. })
    ));
    selection.device = Some(GraphicsDeviceKey {
        vendor: 0,
        device: 0,
        name: "missing".into(),
    });
    assert!(matches!(
        resolve(&[], &selection),
        Err(RenderSurfaceError::GraphicsSelectionUnavailable { .. })
    ));
}

#[test]
fn never_associates_unknown_opengl_id_with_multiple_model_candidates() {
    let first = record(GraphicsBackend::Vulkan);
    let mut second = first.clone();
    second.key.device = 999;
    let mut gl = record(GraphicsBackend::OpenGl);
    gl.key.device = 0;
    let devices = graphics_devices(&[first, second, gl]);
    assert_eq!(devices.len(), 3);
}
