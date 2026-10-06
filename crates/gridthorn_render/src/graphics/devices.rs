use super::{
    GraphicsAdapter, GraphicsBackend, GraphicsDevice, GraphicsDeviceKey, GraphicsSelection,
};
use crate::{GraphicsAdapterCompatibility, RenderSurfaceError};

impl GraphicsSelection {
    /// Resolve independent preferences against a surface-specific inventory.
    /// Returns `None` when both choices are automatic. No native device is created.
    ///
    /// # Errors
    /// Returns unavailable, ambiguous or incompatible selection errors.
    pub fn resolve(
        &self,
        adapters: &[GraphicsAdapter],
    ) -> Result<Option<super::GraphicsAdapterKey>, RenderSurfaceError> {
        resolve(adapters, self)
    }
}

fn model_name(adapter: &GraphicsAdapter) -> &str {
    if adapter.key.backend == GraphicsBackend::OpenGl {
        adapter
            .key
            .name
            .strip_suffix("/PCIe/SSE2")
            .unwrap_or(&adapter.key.name)
    } else {
        &adapter.key.name
    }
}

/// Group backend records into revalidated device-model preferences.
/// Duplicate records for one API remain visible and explicitly ambiguous.
/// These groups are not guaranteed physical-device identities.
#[must_use]
pub fn graphics_devices(adapters: &[GraphicsAdapter]) -> Vec<GraphicsDevice> {
    let mut devices: Vec<GraphicsDevice> = Vec::new();
    for adapter in adapters
        .iter()
        .filter(|adapter| adapter.key.backend != GraphicsBackend::OpenGl)
    {
        insert(&mut devices, adapter, adapter.key.device);
    }
    for adapter in adapters
        .iter()
        .filter(|adapter| adapter.key.backend == GraphicsBackend::OpenGl)
    {
        let candidates: Vec<_> = devices
            .iter()
            .filter(|device| {
                device.key.vendor == adapter.key.vendor
                    && device.key.name == model_name(adapter)
                    && (adapter.key.device == 0 || adapter.key.device == device.key.device)
            })
            .map(|device| device.key.device)
            .collect();
        let device = if candidates.len() == 1 {
            candidates[0]
        } else {
            adapter.key.device
        };
        insert(&mut devices, adapter, device);
    }
    devices
}

fn insert(devices: &mut Vec<GraphicsDevice>, adapter: &GraphicsAdapter, device: u32) {
    let key = GraphicsDeviceKey {
        vendor: adapter.key.vendor,
        device,
        name: model_name(adapter).into(),
    };
    if let Some(existing) = devices.iter_mut().find(|existing| existing.key == key) {
        existing.apis.push(adapter.clone());
    } else {
        devices.push(GraphicsDevice {
            key,
            apis: vec![adapter.clone()],
        });
    }
}

pub(crate) fn resolve(
    adapters: &[GraphicsAdapter],
    selection: &GraphicsSelection,
) -> Result<Option<super::GraphicsAdapterKey>, RenderSurfaceError> {
    if selection.device.is_none() && selection.api.is_none() {
        return Ok(None);
    }
    let devices = graphics_devices(adapters);
    let candidates: Vec<_> = if let Some(key) = &selection.device {
        devices
            .iter()
            .find(|device| &device.key == key)
            .ok_or_else(|| RenderSurfaceError::GraphicsSelectionUnavailable {
                selection: selection.clone(),
            })?
            .apis
            .iter()
            .collect()
    } else {
        adapters.iter().collect()
    };
    let candidates: Vec<_> = candidates
        .into_iter()
        .filter(|adapter| selection.api.is_none_or(|api| adapter.key.backend == api))
        .collect();
    let candidate = candidates
        .iter()
        .find(|adapter| adapter.compatibility == GraphicsAdapterCompatibility::Compatible)
        .or_else(|| candidates.first())
        .ok_or_else(|| RenderSurfaceError::GraphicsSelectionUnavailable {
            selection: selection.clone(),
        })?;
    if candidates
        .iter()
        .filter(|adapter| adapter.key.backend == candidate.key.backend)
        .count()
        > 1
        && selection.device.is_some()
    {
        return Err(RenderSurfaceError::GraphicsSelectionAmbiguous {
            selection: selection.clone(),
        });
    }
    super::select(adapters, &candidate.key)?;
    Ok(Some(candidate.key.clone()))
}
