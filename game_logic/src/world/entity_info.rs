use super::GridCoords;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
    Miner,
}

#[derive(Debug)]
pub struct EntityInfo {
    pub position: GridCoords,
    pub entity_type: EntityType,
}
