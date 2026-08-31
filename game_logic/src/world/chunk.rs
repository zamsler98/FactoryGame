use game_core::Entity;

#[derive(Debug, Default)]
pub struct Chunk {
    pub entities: Vec<Entity>,
    pub coords: ChunkCoords,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkCoords {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Chunk {
    pub fn new(coords: ChunkCoords) -> Self {
        Self {
            entities: Vec::new(),
            coords,
        }
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}
