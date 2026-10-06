use super::super::{
    GraphicsAdapter, GraphicsAdapterCompatibility, GraphicsAdapterKey, GraphicsBackend, select,
};
use crate::RenderSurfaceError;

fn adapter(compatibility: GraphicsAdapterCompatibility) -> GraphicsAdapter {
    GraphicsAdapter {
        key: GraphicsAdapterKey {
            backend: GraphicsBackend::Direct3D12,
            vendor: 1,
            device: 2,
            name: "test GPU".into(),
        },
        software: false,
        compatibility,
    }
}

#[test]
fn selects_only_the_requested_compatible_adapter() {
    let requested = adapter(GraphicsAdapterCompatibility::Compatible);
    let mut other = requested.clone();
    other.key.backend = GraphicsBackend::Vulkan;
    assert_eq!(
        select(&[other, requested.clone()], &requested.key).unwrap(),
        1
    );
}

#[test]
fn rejects_missing_and_ambiguous_preferences_without_fallback() {
    let requested = adapter(GraphicsAdapterCompatibility::Compatible);
    assert!(matches!(
        select(&[], &requested.key),
        Err(RenderSurfaceError::AdapterUnavailable { .. })
    ));
    assert!(matches!(
        select(&[requested.clone(), requested.clone()], &requested.key),
        Err(RenderSurfaceError::AdapterAmbiguous { .. })
    ));
}

#[test]
fn reports_each_incompatibility_reason() {
    for reason in [
        GraphicsAdapterCompatibility::UnsupportedLimits,
        GraphicsAdapterCompatibility::UnsupportedSurface,
        GraphicsAdapterCompatibility::RequiresSurfaceValidation,
    ] {
        let requested = adapter(reason);
        let error = select(std::slice::from_ref(&requested), &requested.key).unwrap_err();
        assert!(
            matches!(error, RenderSurfaceError::AdapterIncompatible { reason: actual, .. } if actual == reason)
        );
        assert!(error.to_string().contains("test GPU"));
    }
}
