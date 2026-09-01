use game_core::Vec2i;

/// A location in world units.
pub type Position = Vec2i;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
    Miner,
}

#[derive(Debug)]
pub struct EntityInfo {
    pub position: Position,
    pub entity_type: EntityType,
}
