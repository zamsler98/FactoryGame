#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
    Miner,
}

#[derive(Debug)]
pub struct EntityInfo {
    pub position: Position,
    pub entity_type: EntityType,
}
