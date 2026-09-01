//! Math primitives shared by every crate. Platform-agnostic by construction:
//! `game_core` cannot depend on Macroquad, so these stand in for its `Vec2`.

mod vec2;

pub use vec2::{Vec2, Vec2f, Vec2i};
