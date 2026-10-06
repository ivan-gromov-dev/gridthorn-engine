mod devices;
mod inventory;
mod types;

pub use devices::graphics_devices;
pub(crate) use devices::resolve;
pub use inventory::enumerate_graphics_adapters;
pub(crate) use inventory::{describe, select};
pub use types::{
    GraphicsAdapter, GraphicsAdapterCompatibility, GraphicsAdapterKey, GraphicsAdapters,
    GraphicsBackend, GraphicsDevice, GraphicsDeviceKey, GraphicsSelection,
};

#[cfg(test)]
mod test;
