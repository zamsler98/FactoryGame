use super::entity::Entity;

#[derive(Default)]
pub struct EntityHolder {
    generations: Vec<u32>,
    free: Vec<u32>,
}

impl EntityHolder {
    pub fn create_entity(&mut self) -> Entity {
        if self.free.len() > 0 {
            let free_index = self.free.pop().expect("just checked before");
            Entity::new(free_index, self.generations[free_index as usize])
        } else {
            let new_index = self.generations.len();
            self.generations.resize(new_index + 1, 0);
            Entity::new(new_index as u32, 0)
        }
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        if let Some(generation) = self.generations.get_mut(entity.index as usize) {
            if *generation == entity.generation {
                *generation += 1;
                self.free.push(entity.index);
            }
        }
    }

    pub fn is_valid(&self, entity: Entity) -> bool {
        self.generations.get(entity.index as usize) == Some(&entity.generation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_entity() {
        let mut holder = EntityHolder::default();
        let entity = holder.create_entity();
        assert_eq!(entity.index, 0);
        assert_eq!(entity.generation, 0);
    }

    #[test]
    fn is_valid_return_true_when_valid() {
        let mut holder = EntityHolder::default();
        let entity = holder.create_entity();
        let res = holder.is_valid(entity);
        assert!(res)
    }

    #[test]
    fn is_valid_return_false_when_index_different() {
        let mut holder = EntityHolder::default();
        holder.create_entity();
        let entity = Entity::new(1, 0);
        let res = holder.is_valid(entity);
        assert!(!res);
    }

    #[test]
    fn is_valid_return_false_when_gen_diff() {
        let mut holder = EntityHolder::default();
        let entity = holder.create_entity();
        let entity = Entity::new(entity.index, entity.generation + 1);
        assert!(!holder.is_valid(entity))
    }

    #[test]
    fn delete_entity_make_not_valid() {
        let mut holder = EntityHolder::default();
        let entity = holder.create_entity();
        holder.delete_entity(entity);
        assert!(!holder.is_valid(entity))
    }

    #[test]
    fn delete_entity_do_nothing_entity_not_exist() {
        let mut holder = EntityHolder::default();
        let entity = holder.create_entity();
        let entity2 = Entity::new(1, 0);
        holder.delete_entity(entity2);
        assert!(holder.is_valid(entity))
    }

    #[test]
    fn create_entity_reuse_slot() {
        let mut holder = EntityHolder::default();
        let entity = holder.create_entity();
        let entity2 = holder.create_entity();
        holder.delete_entity(entity);
        let entity = holder.create_entity();
        assert!(holder.is_valid(entity));
        assert!(holder.is_valid(entity2));
        assert_eq!(entity.generation, 1);
        assert_eq!(entity.index, 0);
    }

    #[test]
    fn old_generation_not_remove_entity() {
        let mut holder = EntityHolder::default();
        let entity = holder.create_entity();
        holder.delete_entity(entity);
        let entity2 = holder.create_entity();
        holder.delete_entity(entity);
        assert!(holder.is_valid(entity2));
    }

    #[test]
    fn delete_entity_twice() {
        let mut holder = EntityHolder::default();
        let entity = holder.create_entity();
        holder.delete_entity(entity);
        holder.delete_entity(entity);
        assert!(!holder.is_valid(entity))
    }
}
