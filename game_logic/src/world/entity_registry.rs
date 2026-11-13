use game_core::{Entity, EntityAllocator, EntitySparseSet};

use crate::Building;

#[derive(Default)]
pub struct EntityRegistry {
    allocator: EntityAllocator,
    pub buildings: EntitySparseSet<Building>,
}

impl EntityRegistry {
    pub fn spawn(&mut self) -> Entity {
        self.allocator.spawn()
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.allocator.despawn(entity);
        self.buildings.remove(entity);
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.allocator.is_alive(entity)
    }
}
