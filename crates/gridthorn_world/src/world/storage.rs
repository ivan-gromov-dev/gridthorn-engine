use bevy_ecs::{component::Component, resource::Resource};

#[derive(Component)]
pub(crate) struct StoredComponent<T: Send + Sync + 'static>(pub(crate) T);

#[derive(Resource)]
pub(crate) struct StoredResource<T: Send + Sync + 'static>(pub(crate) T);
