use crate::world::entity_info::EntityInfo;

#[derive(Debug, Default)]
pub struct Chunk {
    pub entities: Vec<EntityInfo>,
    pub coords: ChunkCoords,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkCoords {
    pub x: i32,
    pub y: i32,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            coords: ChunkCoords::default(),
        }
    }
    pub fn add_entity(&mut self, entity: EntityInfo) {
        self.entities.push(entity);
    }
}
