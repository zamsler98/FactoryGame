use super::entity::Entity;

#[derive(Default)]
pub struct EntityAllocator {
    generations: Vec<u32>,
    free_indices: Vec<u32>,
}

impl EntityAllocator {
    pub fn spawn(&mut self) -> Entity {
        if let Some(index) = self.free_indices.pop() {
            Entity::new(index, self.generations[index as usize])
        } else {
            let index = self.generations.len();
            self.generations.push(0);
            Entity::new(index as u32, 0)
        }
    }

    pub fn despawn(&mut self, entity: Entity) {
        if let Some(generation) = self.generations.get_mut(entity.index as usize) {
            if *generation == entity.generation {
                *generation += 1;
                self.free_indices.push(entity.index);
            }
        }
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.generations.get(entity.index as usize) == Some(&entity.generation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_returns_first_entity() {
        let mut allocator = EntityAllocator::default();
        let entity = allocator.spawn();
        assert_eq!(entity.index, 0);
        assert_eq!(entity.generation, 0);
    }

    #[test]
    fn spawned_entity_is_alive() {
        let mut allocator = EntityAllocator::default();
        let entity = allocator.spawn();
        assert!(allocator.is_alive(entity));
    }

    #[test]
    fn entity_with_unknown_index_is_not_alive() {
        let mut allocator = EntityAllocator::default();
        allocator.spawn();
        let entity = Entity::new(1, 0);
        assert!(!allocator.is_alive(entity));
    }

    #[test]
    fn entity_with_different_generation_is_not_alive() {
        let mut allocator = EntityAllocator::default();
        let entity = allocator.spawn();
        let entity = Entity::new(entity.index, entity.generation + 1);
        assert!(!allocator.is_alive(entity));
    }

    #[test]
    fn despawn_marks_entity_dead() {
        let mut allocator = EntityAllocator::default();
        let entity = allocator.spawn();
        allocator.despawn(entity);
        assert!(!allocator.is_alive(entity));
    }

    #[test]
    fn despawning_unknown_entity_is_noop() {
        let mut allocator = EntityAllocator::default();
        let entity = allocator.spawn();
        let unknown = Entity::new(1, 0);
        allocator.despawn(unknown);
        assert!(allocator.is_alive(entity));
    }

    #[test]
    fn spawn_reuses_freed_slot() {
        let mut allocator = EntityAllocator::default();
        let entity = allocator.spawn();
        let other = allocator.spawn();
        allocator.despawn(entity);
        let reused = allocator.spawn();
        assert!(allocator.is_alive(reused));
        assert!(allocator.is_alive(other));
        assert_eq!(reused.generation, 1);
        assert_eq!(reused.index, 0);
    }

    #[test]
    fn stale_entity_does_not_despawn_reused_entity() {
        let mut allocator = EntityAllocator::default();
        let stale = allocator.spawn();
        allocator.despawn(stale);
        let current = allocator.spawn();
        allocator.despawn(stale);
        assert!(allocator.is_alive(current));
    }

    #[test]
    fn despawning_entity_twice_is_noop() {
        let mut allocator = EntityAllocator::default();
        let entity = allocator.spawn();
        allocator.despawn(entity);
        allocator.despawn(entity);
        assert!(!allocator.is_alive(entity));
    }
}
