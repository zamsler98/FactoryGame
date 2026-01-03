//! game_logic: processes inputs, game rules, and AI.
//! Depends on `game_core` only. It exposes an `InputFrame` and `update_world`.

use game_core::{EntityType, World};

/// `InputFrame` is the platform-agnostic input snapshot.
/// The platform layer (`game_app`) fills this each frame and passes to logic.
#[derive(Clone, Debug)]
pub struct InputFrame {
    /// Movement direction [-1.0, 1.0] on X
    pub move_x: f32,
    /// Movement direction [-1.0, 1.0] on Y
    pub move_y: f32,
    /// Whether the primary action (e.g., fire) was pressed this frame
    pub action: bool,
    /// Optional pointer/touch position in world / screen coords
    pub pointer: Option<(f32, f32)>,
}

impl Default for InputFrame {
    fn default() -> Self {
        Self {
            move_x: 0.0,
            move_y: 0.0,
            action: false,
            pointer: None,
        }
    }
}

/// Update the world based on the `input` and a timestep `dt`.
///
/// - moves player by setting its velocity from input
/// - updates enemy behavior (very simple: move toward player)
///
/// Note: This function does not render or call Macroquad.
pub fn update_world(world: &mut World, input: &InputFrame, dt: f32) {
    const PLAYER_SPEED: f32 = 180.0;
    const ENEMY_SPEED: f32 = 80.0;

    // Apply player input by setting velocity on the player entity.
    if let Some(player) = world.find_player_mut() {
        player.velocity.vx = input.move_x * PLAYER_SPEED;
        player.velocity.vy = input.move_y * PLAYER_SPEED;

        // Example: if action pressed, do something. Here we just print for headless logs.
        if input.action {
            // In a real game we'd spawn bullets or trigger actions.
            // Keep pure: don't call platform logging here.
            // You could return an event enum from this function if needed.
        }
    }

    // Simple enemy AI: move toward player
    if let Some(player_pos) = world.find_player().map(|p| (p.transform.x, p.transform.y)) {
        for e in &mut world.entities {
            if e.ty == EntityType::Enemy {
                let dx = player_pos.0 - e.transform.x;
                let dy = player_pos.1 - e.transform.y;
                let dist = (dx * dx + dy * dy).sqrt().max(0.001);
                let nx = dx / dist;
                let ny = dy / dist;
                e.velocity.vx = nx * ENEMY_SPEED;
                e.velocity.vy = ny * ENEMY_SPEED;
            }
        }
    }

    // Integrate physics for positions (game_core provides deterministic integration).
    world.update_physics(dt);
}

/// Optional: an abstract drawing trait that UI/app can implement if desired.
/// game_logic can provide high-level debug draw calls using this trait (optional).
pub trait DrawBackend {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, rgba: (f32, f32, f32, f32));
}
