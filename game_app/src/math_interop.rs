//! Conversions between `game_core`'s `Vec2` and Macroquad's.
//!
//! We hold our own `Vec2f` in app state and convert only where a Macroquad API
//! demands its type. The orphan rule forbids `impl From<Vec2f> for
//! macroquad::math::Vec2` here — both types are foreign to this crate — so the
//! conversions hang off locally-defined traits instead.

use game_core::Vec2f;
use macroquad::math::{Vec2, vec2};

/// Converts into Macroquad's `Vec2`, for handing our state to a draw call.
pub trait ToMacroquad {
    fn to_macroquad(self) -> Vec2;
}

impl ToMacroquad for Vec2f {
    fn to_macroquad(self) -> Vec2 {
        vec2(self.x, self.y)
    }
}

/// Converts out of Macroquad's `Vec2`, for taking a result back into our state.
#[expect(
    dead_code,
    reason = "only reached from MainCamera::bounds, which is not wired up yet"
)]
pub trait ToCore {
    fn to_core(self) -> Vec2f;
}

impl ToCore for Vec2 {
    fn to_core(self) -> Vec2f {
        Vec2f::new(self.x, self.y)
    }
}
