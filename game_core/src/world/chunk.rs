use crate::entity::Entity;

#[derive(Debug)]
pub struct Chunk {
    pub entities: Vec<Entity>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
