use super::{GraphicsAdapter, GraphicsAdapterCompatibility, GraphicsAdapterKey, GraphicsBackend};
use crate::RenderSurfaceError;

/// Enumerate native adapters without opening a window or requesting devices.
/// An empty list means no adapter was exposed. Surface support is checked at initialization.
/// Ordering and keys are backend observations and must be revalidated after restart.
#[must_use]
pub fn enumerate_graphics_adapters() -> Vec<GraphicsAdapter> {
    let instance = wgpu::Instance::default();
    pollster::block_on(instance.enumerate_adapters(wgpu::Backends::all()))
        .iter()
        .map(|adapter| describe(adapter, None))
        .collect()
}

pub(crate) fn describe(
    adapter: &wgpu::Adapter,
    surface: Option<&wgpu::Surface<'_>>,
) -> GraphicsAdapter {
    let info = adapter.get_info();
    let compatibility = if !wgpu::Limits::default().check_limits(&adapter.limits()) {
        GraphicsAdapterCompatibility::UnsupportedLimits
    } else if let Some(surface) = surface {
        if adapter.is_surface_supported(surface)
            && surface.get_default_config(adapter, 1, 1).is_some()
        {
            GraphicsAdapterCompatibility::Compatible
        } else {
            GraphicsAdapterCompatibility::UnsupportedSurface
        }
    } else {
        GraphicsAdapterCompatibility::RequiresSurfaceValidation
    };
    GraphicsAdapter {
        key: GraphicsAdapterKey {
            backend: match info.backend {
                wgpu::Backend::Vulkan => GraphicsBackend::Vulkan,
                wgpu::Backend::Dx12 => GraphicsBackend::Direct3D12,
                wgpu::Backend::Metal => GraphicsBackend::Metal,
                wgpu::Backend::Gl => GraphicsBackend::OpenGl,
                _ => GraphicsBackend::Other,
            },
            vendor: info.vendor,
            device: info.device,
            name: info.name,
        },
        software: info.device_type == wgpu::DeviceType::Cpu,
        compatibility,
    }
}

pub(crate) fn select(
    adapters: &[GraphicsAdapter],
    key: &GraphicsAdapterKey,
) -> Result<usize, RenderSurfaceError> {
    let mut matches = adapters
        .iter()
        .enumerate()
        .filter(|(_, adapter)| &adapter.key == key);
    let Some((index, adapter)) = matches.next() else {
        return Err(RenderSurfaceError::AdapterUnavailable { key: key.clone() });
    };
    if matches.next().is_some() {
        return Err(RenderSurfaceError::AdapterAmbiguous { key: key.clone() });
    }
    if adapter.compatibility != GraphicsAdapterCompatibility::Compatible {
        return Err(RenderSurfaceError::AdapterIncompatible {
            key: key.clone(),
            reason: adapter.compatibility,
        });
    }
    Ok(index)
}
