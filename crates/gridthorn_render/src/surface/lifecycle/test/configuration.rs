use super::*;

#[test]
fn startup_resize_events_coalesce_until_the_first_render() {
    let mut lifecycle = SurfaceLifecycle::default();
    assert_eq!(
        lifecycle.resize(960, 540),
        SurfaceChange::QueueConfiguration(SurfaceExtent {
            width: 960,
            height: 540
        })
    );
    assert!(lifecycle.can_render());
    lifecycle.resize(1424, 714);
    lifecycle.resize(960, 540);
    assert_eq!(
        lifecycle.configuration_required(),
        Some(SurfaceExtent {
            width: 960,
            height: 540
        })
    );
    assert_eq!(lifecycle.resize(960, 540), SurfaceChange::Unchanged);
    assert!(lifecycle.configuration_required().is_some());
    lifecycle.mark_configured();
    assert_eq!(lifecycle.configuration_required(), None);
    lifecycle.invalidate_configuration();
    assert_eq!(
        lifecycle.configuration_required(),
        Some(SurfaceExtent {
            width: 960,
            height: 540
        })
    );
    lifecycle.mark_configured();
    assert_eq!(lifecycle.resize(960, 540), SurfaceChange::Unchanged);
    assert_eq!(lifecycle.configuration_required(), None);
    lifecycle.resize(800, 600);
    assert_eq!(
        lifecycle.configuration_required(),
        Some(SurfaceExtent {
            width: 800,
            height: 600
        })
    );
}

#[test]
fn minimized_and_occluded_windows_do_not_configure_swapchains() {
    let mut lifecycle = SurfaceLifecycle::default();
    lifecycle.resize(960, 540);
    lifecycle.set_occluded(true);
    assert!(!lifecycle.can_render());
    assert_eq!(
        lifecycle.extent(),
        Some(SurfaceExtent {
            width: 960,
            height: 540
        })
    );
    assert_eq!(lifecycle.configuration_required(), None);
    lifecycle.resize(800, 600);
    lifecycle.set_occluded(false);
    assert!(lifecycle.can_render());
    assert_eq!(
        lifecycle.configuration_required(),
        Some(SurfaceExtent {
            width: 800,
            height: 600
        })
    );
    assert_eq!(lifecycle.resize(0, 0), SurfaceChange::Suspend);
    assert!(!lifecycle.can_render());
    assert_eq!(lifecycle.extent(), None);
    assert_eq!(lifecycle.configuration_required(), None);
    lifecycle.resize(640, 480);
    assert_eq!(
        lifecycle.configuration_required(),
        Some(SurfaceExtent {
            width: 640,
            height: 480
        })
    );
}
