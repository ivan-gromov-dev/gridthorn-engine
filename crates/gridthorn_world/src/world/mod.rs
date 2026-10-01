mod access;
mod entity;
mod scene_storage;
mod storage;

pub use access::WorldAccess;
pub use entity::EntityId;

pub(crate) use storage::{StoredComponent, StoredResource};

#[cfg(test)]
mod test;
