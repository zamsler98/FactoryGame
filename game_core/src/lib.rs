//! game_core: pure game state.
//! This crate must not depend on Macroquad or any platform APIs.
//! It contains the world and deterministic update functions.

mod entity;
pub mod utilities;

pub use entity::Entity;
