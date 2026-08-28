/// Stable entity identifier owned by the Gridthorn world boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntityId(pub(crate) bevy_ecs::entity::Entity);
