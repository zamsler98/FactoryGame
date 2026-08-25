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
        self.generations[entity.index as usize] += 1;
        self.free.push(entity.index);
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

    // #[test]
    // fn remove_entity() {
    //     assert!(false)
    // }
}
