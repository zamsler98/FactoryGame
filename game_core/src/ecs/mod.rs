//! ecs: the entity-component system.
//!
//! An `Entity` is a generational handle that owns no data. Component data lives
//! in per-type storages keyed by that handle, currently `EntitySparseSet<T>`.

mod entity;
mod entity_allocator;
mod entity_sparse_set;

pub use entity::Entity;
pub use entity_allocator::EntityAllocator;
pub use entity_sparse_set::EntitySparseSet;
