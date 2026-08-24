#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId {
    pub index: u32,
    generation: u32,
}

impl EntityId {
    pub fn new(index: u32, generation: u32) -> Self {
        Self {
            index: index,
            generation: generation,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
    Miner,
}

#[derive(Debug)]
pub struct Entity {
    pub position: Position,
    pub entity_type: EntityType,
}
