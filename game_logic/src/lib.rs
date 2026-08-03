//! game_logic: processes inputs, game rules, and building placement.
//! Depends on `game_core` only. It exposes an `InputFrame`.

use game_core::World;

/// `InputFrame` is the platform-agnostic input snapshot.
/// The platform layer (`game_app`) fills this each frame and passes to logic.
#[derive(Clone, Debug, Default)]
pub struct InputFrame {
    /// Whether the primary action (e.g., tap/click) was pressed this frame
    pub action: bool,
    /// Optional pointer/touch position in world / screen coords
    pub pointer: Option<(f32, f32)>,
}

/// Advance the game rules by `dt` seconds: steps the factory simulation and
/// the player's hand-crafting queue. Input-driven placement is handled by the
/// platform layer via `placement`.
pub fn update_world(world: &mut World, _input: &InputFrame, dt: f32) {
    world.update(dt);
}

pub mod placement;
pub mod view;
