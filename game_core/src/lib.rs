//! game_core: pure game state.
//! This crate must not depend on Macroquad or any platform APIs.
//! It contains the entity-component system and deterministic update functions.

pub mod ecs;

pub use ecs::{Entity, EntitySparseSet};
