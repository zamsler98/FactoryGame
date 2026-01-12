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
pub mod placement;

pub trait DrawBackend {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, rgba: (f32, f32, f32, f32));
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
<<<<<<< HEAD
=======
use std::sync::{Mutex, OnceLock};

pub const ITEM_RADIUS: f32 = 8.0;
pub const MINER_SPAWN_INTERVAL: f32 = 1.0; // seconds
pub const TILE_SIZE: f32 = 32.0;
pub const CONVEYOR_SPEED: f32 = 64.0; // units/sec

// simple building types by spec id (1=conveyor,2=miner,3=smelter)
#[derive(Clone, Debug)]
struct MinerState {
    pub spawn_accum: f32,
    pub output_pos: (f32, f32),
}

#[derive(Clone, Debug)]
struct ConveyorSpec {
    pub tile: TilePos,
    pub direction: (f32, f32),
}

#[derive(Clone, Debug)]
struct SmelterSpec {
    pub tile: TilePos,
}

// world-local simple registries (pure logic; not serialized)
#[derive(Default)]
struct LogicState {
    pub miners: Vec<MinerState>,
    pub conveyors: Vec<ConveyorSpec>,
    pub smelters: Vec<SmelterSpec>,
}

// simple global logic state per process (keeps example small). In a full game this would live in World.
static LOGIC_STATE: OnceLock<Mutex<LogicState>> = OnceLock::new();

/// Ensure logic state exists and return a locked guard for mutable access.
fn ensure_logic_state() -> std::sync::MutexGuard<'static, LogicState> {
    LOGIC_STATE
        .get_or_init(|| Mutex::new(LogicState::default()))
        .lock()
        .expect("Logic state mutex poisoned")
}

pub fn register_example_buildings(world: &World) {
    // For demo purposes we scan tile grid for instances and register conveyors/miners/smelters
    // This function is cheap-ish and idempotent; we keep a simple snapshot in LOGIC_STATE.
    let mut state = ensure_logic_state();
    state.miners.clear();
    state.conveyors.clear();
    state.smelters.clear();

    for inst in world.tile_grid.instances.values() {
        match inst.spec_id {
            1 => {
                // conveyor: determine direction from rotation
                let dir = match inst.rotation {
                    game_core::Rotation::R0 => (1.0, 0.0),
                    game_core::Rotation::R90 => (0.0, 1.0),
                    game_core::Rotation::R180 => (-1.0, 0.0),
                    game_core::Rotation::R270 => (0.0, -1.0),
                };
                state.conveyors.push(ConveyorSpec {
                    tile: inst.origin,
                    direction: dir,
                });
            }
            2 => {
                // miner: spawn at center of tile
                let x = (inst.origin.x as f32 + 0.5) * TILE_SIZE;
                let y = (inst.origin.y as f32 + 0.5) * TILE_SIZE;
                state.miners.push(MinerState {
                    spawn_accum: 0.0,
                    output_pos: (x, y),
                });
            }
            3 => {
                state.smelters.push(SmelterSpec { tile: inst.origin });
            }
            _ => {}
        }
    }
}

/// Optional: an abstract drawing trait that UI/app can implement if desired.
/// game_logic can provide high-level debug draw calls using this trait (optional).
pub mod placement;

pub trait DrawBackend {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, rgba: (f32, f32, f32, f32));
}

/// Update the world based on the `input` and a timestep `dt`.
>>>>>>> parent of e301aca (feat: implement item flow (miner → conveyor → smelter) and rendering)
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
pub mod placement;

pub trait DrawBackend {
    fn draw_circle(&mut self, x: f32, y: f32, radius: f32, rgba: (f32, f32, f32, f32));
}
