use super::entity::Entity;

const EMPTY: u32 = u32::MAX;

pub struct EntitySparseSet<T> {
    sparse: Vec<u32>,
    dense: Vec<T>,
    owners: Vec<Entity>,
}

impl<T> Default for EntitySparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            owners: Vec::new(),
        }
    }
}

impl<T> EntitySparseSet<T> {
    pub fn insert(&mut self, id: Entity, value: T) {
        let index = id.index as usize;
        if index >= self.sparse.len() {
            self.sparse.resize(index + 1, EMPTY)
        }
        if self.sparse[index] == EMPTY {
            self.sparse[index] = self.dense.len() as u32;
            self.dense.push(value);
            self.owners.push(id);
        } else {
            let dense_index = self.sparse[index] as usize;
            self.dense[dense_index] = value;
            self.owners[dense_index] = id;
        }
    }

    pub fn remove(&mut self, id: Entity) -> Option<T> {
        let dense_index = self.get_dense_index(id)?;
        self.sparse[id.index as usize] = EMPTY;
        let value = self.dense.swap_remove(dense_index);
        self.owners.swap_remove(dense_index);
        if dense_index < self.owners.len() {
            let moved = self.owners[dense_index];
            self.sparse[moved.index as usize] = dense_index as u32;
        }
        Some(value)
    }

    pub fn get(&self, id: Entity) -> Option<&T> {
        let dense_index = self.get_dense_index(id)?;
        self.dense.get(dense_index)
    }

    fn get_dense_index(&self, id: Entity) -> Option<usize> {
        let dense_index = *self.sparse.get(id.index as usize)?;
        if dense_index == EMPTY {
            return None;
        }
        let dense_index = dense_index as usize;
        (self.owners[dense_index] == id).then_some(dense_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_does_not_require_component_default() {
        struct Component;

        let _: EntitySparseSet<Component> = EntitySparseSet::default();
    }

    #[test]
    fn insert_entity() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(0, 0);
        set.insert(id, to_store);
        let stored = set.get(id).expect("entity should be stored");
        assert_eq!(*stored, to_store);
    }

    #[test]
    fn insert_entity_not_next() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(10, 0);
        set.insert(id, to_store);
        let stored = set.get(id).expect("entity should be stored");
        assert_eq!(*stored, to_store);
    }

    #[test]
    fn insert_overwrite_old_slot() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(10, 0);
        set.insert(id, 37);
        set.insert(id, to_store);
        let stored = set.get(id).expect("entity should be stored");
        assert_eq!(*stored, to_store);
    }

    #[test]
    fn remove_entity() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(10, 0);
        set.insert(id, to_store);
        let removed = set.remove(id).expect("removed value should be returned");
        assert_eq!(removed, to_store);
        let stored = set.get(id);
        assert_eq!(stored, None);
    }

    #[test]
    fn remove_entity_does_not_exist() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let id = Entity::new(10, 0);
        let removed = set.remove(id);
        assert_eq!(removed, None);
        let stored = set.get(id);
        assert_eq!(stored, None);
    }

    #[test]
    fn reuse_old_index() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let id = Entity::new(10, 0);
        let to_store: u32 = 42;
        set.insert(id, 32);
        set.remove(id);
        set.insert(id, to_store);
        let stored = set.get(id).expect("should return new stored value");
        assert_eq!(*stored, to_store);
    }

    #[test]
    fn get_new_generation() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(10, 0);
        set.insert(id, to_store);
        let id2 = Entity::new(10, 1);
        let stored = set.get(id2);
        assert_eq!(stored, None);
    }

    #[test]
    fn insert_new_generation() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(10, 0);
        set.insert(id, 37);
        let id2 = Entity::new(10, 1);
        set.insert(id2, to_store);
        let get_from_id_1 = set.get(id);
        assert_eq!(get_from_id_1, None);
        let get_from_id_2 = set.get(id2).expect("id2 should have result stored");
        assert_eq!(*get_from_id_2, to_store);
    }

    #[test]
    fn remove_non_last_element() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(0, 0);
        let id2 = Entity::new(1, 0);
        set.insert(id, 37);
        set.insert(id2, to_store);

        assert_eq!(set.remove(id), Some(37));
        assert_eq!(set.get(id), None);
        assert_eq!(set.get(id2), Some(&to_store));

        set.insert(id, 37);
        assert_eq!(set.get(id), Some(&37));
        assert_eq!(set.get(id2), Some(&to_store));
    }

    #[test]
    fn remove_id_twice() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(0, 0);
        set.insert(id, to_store);
        set.remove(id);
        let value = set.remove(id);
        assert_eq!(value, None);
    }

    #[test]
    fn remove_stale_generation() {
        let mut set: EntitySparseSet<u32> = EntitySparseSet::default();
        let to_store: u32 = 42;
        let id = Entity::new(0, 0);
        let id2 = Entity::new(0, 1);
        set.insert(id, to_store);
        let value = set.remove(id2);
        assert_eq!(value, None);
        let stored = set.get(id).expect("value should be returned");
        assert_eq!(*stored, to_store)
    }
}
