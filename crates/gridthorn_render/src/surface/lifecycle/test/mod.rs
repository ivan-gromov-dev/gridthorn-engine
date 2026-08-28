use super::{SurfaceChange, SurfaceExtent, SurfaceLifecycle};

#[test]
fn configures_for_a_non_zero_extent() {
    let mut lifecycle = SurfaceLifecycle::default();

    assert_eq!(
        lifecycle.resize(1280, 720),
        SurfaceChange::Configure(SurfaceExtent {
            width: 1280,
            height: 720
        })
    );
    assert!(lifecycle.can_render());
}

#[test]
fn suspends_for_zero_sized_minimized_surface() {
    let mut lifecycle = SurfaceLifecycle::default();
    lifecycle.resize(1280, 720);

    assert_eq!(lifecycle.resize(0, 0), SurfaceChange::Suspend);
    assert!(!lifecycle.can_render());
    assert_eq!(lifecycle.extent(), None);
}

#[test]
fn avoids_reconfiguring_an_unchanged_extent() {
    let mut lifecycle = SurfaceLifecycle::default();
    lifecycle.resize(800, 600);

    assert_eq!(lifecycle.resize(800, 600), SurfaceChange::Unchanged);
}

#[test]
fn occlusion_pauses_rendering_without_discarding_extent() {
    let mut lifecycle = SurfaceLifecycle::default();
    lifecycle.resize(800, 600);
    lifecycle.set_occluded(true);

    assert!(!lifecycle.can_render());
    assert_eq!(
        lifecycle.extent(),
        Some(SurfaceExtent {
            width: 800,
            height: 600
        })
    );

    lifecycle.set_occluded(false);
    assert!(lifecycle.can_render());
}
