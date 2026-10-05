use super::{catalog::DisplayCatalog, *};

fn observation(key: u32) -> (u32, MonitorInfo) {
    (
        key,
        MonitorInfo {
            id: MonitorId(0, 0),
            name: Some("same display name".into()),
            resolution: DisplayResolution {
                width: 1920,
                height: 1080,
            },
            position: (-1920, 0),
            refresh_rate_millihertz: Some(59940),
            scale_factor: 1.25,
            modes: vec![DisplayMode {
                resolution: DisplayResolution {
                    width: 1920,
                    height: 1080,
                },
                refresh_rate_millihertz: 59940,
                bit_depth: 32,
            }],
        },
    )
}

mod identity;
mod modes;
mod requests;
mod selection;
