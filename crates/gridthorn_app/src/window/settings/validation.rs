use super::{WindowMode, WindowOperationError, WindowPlacement, WindowRequest, WindowResizePolicy};
use crate::display::{DisplayResolution, MonitorId};

pub(super) fn valid_size(size: DisplayResolution) -> bool {
    size.width > 0
        && size.height > 0
        && size.width <= i32::MAX.unsigned_abs()
        && size.height <= i32::MAX.unsigned_abs()
}
impl WindowRequest {
    pub(super) fn validate(&self) -> Result<(), WindowOperationError> {
        if *self == Self::default() {
            return Err(WindowOperationError::EmptyRequest);
        }
        if self.size.is_some_and(|size| !valid_size(size)) {
            return Err(WindowOperationError::InvalidSize {
                reason: "client size must be positive and fit desktop coordinates",
            });
        }
        if matches!(
            self.mode,
            Some(WindowMode::Borderless { .. } | WindowMode::Exclusive { .. })
        ) && (self.size.is_some() || self.placement.is_some() || self.resize_policy.is_some())
        {
            return Err(WindowOperationError::WindowedOnly);
        }
        if let Some(WindowMode::Exclusive { mode, .. }) = self.mode
            && !valid_size(mode.resolution)
        {
            return Err(WindowOperationError::InvalidSize {
                reason: "exclusive mode resolution is invalid",
            });
        }
        if let Some(WindowResizePolicy::Resizable { min, max }) = self.resize_policy {
            validate_constraints(min, max, self.size)?;
        }
        Ok(())
    }
    pub(crate) fn monitor(&self) -> Option<MonitorId> {
        match self.mode {
            Some(WindowMode::Borderless { monitor } | WindowMode::Exclusive { monitor, .. }) => {
                Some(monitor)
            }
            _ => match self.placement {
                Some(WindowPlacement::Centered { monitor }) => Some(monitor),
                _ => None,
            },
        }
    }
}

pub(super) fn validate_constraints(
    min: Option<DisplayResolution>,
    max: Option<DisplayResolution>,
    size: Option<DisplayResolution>,
) -> Result<(), WindowOperationError> {
    let invalid = min.is_some_and(|value| !valid_size(value))
        || max.is_some_and(|value| !valid_size(value))
        || matches!((min, max), (Some(min), Some(max)) if min.width > max.width || min.height > max.height)
        || size.is_some_and(|value| {
            min.is_some_and(|min| value.width < min.width || value.height < min.height)
                || max.is_some_and(|max| value.width > max.width || value.height > max.height)
        });
    if invalid {
        Err(WindowOperationError::InvalidSize {
            reason: "min/max bounds or requested size are inconsistent",
        })
    } else {
        Ok(())
    }
}
