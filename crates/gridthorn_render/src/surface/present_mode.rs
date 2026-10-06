/// Explicit surface queue policy. Availability belongs to a surface/adapter pair.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PresentMode {
    /// Ordered queue synchronized to vertical blank; the default `VSync` policy.
    #[default]
    Fifo,
    /// Adaptive `VSync`; late frames may tear.
    FifoRelaxed,
    /// Immediate display with possible tearing.
    Immediate,
    /// Replace queued frames, displaying the latest at vertical blank.
    Mailbox,
}

impl PresentMode {
    pub(super) fn from_native(mode: wgpu::PresentMode) -> Option<Self> {
        match mode {
            wgpu::PresentMode::Fifo => Some(Self::Fifo),
            wgpu::PresentMode::FifoRelaxed => Some(Self::FifoRelaxed),
            wgpu::PresentMode::Immediate => Some(Self::Immediate),
            wgpu::PresentMode::Mailbox => Some(Self::Mailbox),
            wgpu::PresentMode::AutoVsync | wgpu::PresentMode::AutoNoVsync => None,
        }
    }
    pub(super) fn native(self) -> wgpu::PresentMode {
        match self {
            Self::Fifo => wgpu::PresentMode::Fifo,
            Self::FifoRelaxed => wgpu::PresentMode::FifoRelaxed,
            Self::Immediate => wgpu::PresentMode::Immediate,
            Self::Mailbox => wgpu::PresentMode::Mailbox,
        }
    }
}

pub(super) fn supported(modes: &[wgpu::PresentMode]) -> Vec<PresentMode> {
    [
        PresentMode::Fifo,
        PresentMode::FifoRelaxed,
        PresentMode::Immediate,
        PresentMode::Mailbox,
    ]
    .into_iter()
    .filter(|mode| modes.contains(&mode.native()))
    .collect()
}

pub(super) fn validate(
    modes: &[PresentMode],
    mode: PresentMode,
) -> Result<(), super::RenderSurfaceError> {
    if modes.contains(&mode) {
        Ok(())
    } else {
        Err(super::RenderSurfaceError::UnsupportedPresentMode { mode })
    }
}

#[cfg(test)]
#[path = "present_mode/test/mod.rs"]
mod test;
