//! Main entry point for FactoryGame with custom ECS, grid world, and interactive grid rendering.
// use macroquad::input::is_mobile; // Removed: not supported in macroquad 0.4.14+use macroquad::prelude::*;
// Explicit macroquad imports to ensure all are present
use macroquad::prelude::{
    clear_background, draw_rectangle, draw_rectangle_lines, draw_text, is_mouse_button_down,
    is_mouse_button_pressed, mouse_position, mouse_wheel, next_frame, screen_height, screen_width,
    set_camera, set_default_camera, touches, vec2, Camera2D, Color, MouseButton, TouchPhase, Vec2,
    DARKGRAY, DARKGREEN, GRAY, LIGHTGRAY, ORANGE, RED, YELLOW,
};

use std::collections::HashMap;

type EntityId = u32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity(EntityId);

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum TileType {
    Empty,
    Resource,
    Ground,
}

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Clone, Debug)]
pub struct Renderable {
    pub color: Color,
    pub label: Option<String>,
}

/// Core ECS world holding all entity/component state.
pub struct World {
    pub next_entity: EntityId,
    pub positions: HashMap<Entity, Position>,
    pub renderables: HashMap<Entity, Renderable>,
    // More component storages as needed
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity: 0,
            positions: HashMap::new(),
            renderables: HashMap::new(),
        }
    }
    pub fn spawn(&mut self, pos: Position, renderable: Renderable) -> Entity {
        let e = Entity(self.next_entity);
        self.next_entity += 1;
        self.positions.insert(e, pos);
        self.renderables.insert(e, renderable);
        e
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

const GRID_SIZE: usize = 32;
const TILE_SIZE: f32 = 32.0;
const PAN_SPEED: f32 = 0.08;

struct Camera {
    offset: Vec2,
    zoom: f32,
}

#[macroquad::main("FactoryGame")]
async fn main() {
    let mut world = World::new();
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let tile_type = if (x + y) % 8 == 0 {
                TileType::Resource
            } else {
                TileType::Ground
            };
            let color = match tile_type {
                TileType::Ground => DARKGREEN,
                TileType::Resource => YELLOW,
                _ => GRAY,
            };
            world.spawn(Position { x, y }, Renderable { color, label: None });
        }
    }
    let mut camera = Camera {
        offset: Vec2::new(0.0, 0.0),
        zoom: 1.0,
    };
    let mut selected: Option<Position> = None;
    let mut drag_start: Option<Vec2> = None;
    let mut camera_start: Vec2 = Vec2::ZERO;
    let mut pinch_start: Option<(u64, u64)> = None;
    let mut last_pinch_distance = 0.0;
    let mut touch_drag_id: Option<u64> = None;
    let mut last_touch_pos = Vec2::ZERO;
    let mut last_touch_delta = Vec2::ZERO;

    loop {
        clear_background(LIGHTGRAY);

        let touches = touches();
        let mobile = !touches.is_empty(); // Detect mobile/touch if touches exist
                                          // -------- Pan/Drag: Mouse or Touch
        if !mobile {
            // Desktop mouse pan
            if is_mouse_button_pressed(MouseButton::Left) {
                drag_start = Some(vec2(mouse_position().0, mouse_position().1));
                camera_start = camera.offset;
            }
            if is_mouse_button_down(MouseButton::Left) {
                if let Some(start) = drag_start {
                    let curr = vec2(mouse_position().0, mouse_position().1);
                    camera.offset = camera_start + (curr - start) / camera.zoom * PAN_SPEED;
                }
            } else {
                drag_start = None;
            }
        } else {
            // -- Touch drag pan
            if touches.len() == 1 {
                let t = &touches[0];
                if t.phase == TouchPhase::Started {
                    touch_drag_id = Some(t.id);
                    last_touch_pos = t.position;
                    camera_start = camera.offset;
                } else if t.phase == TouchPhase::Moved && touch_drag_id == Some(t.id) {
                    let curr = t.position;
                    let delta = curr - last_touch_pos;
                    // Record delta for debugging on device
                    last_touch_delta = delta;
                    // Fix horizontal scroll inversion: invert delta.x for touch
                    camera.offset =
                        camera_start + vec2(-delta.x, delta.y) / camera.zoom * PAN_SPEED;
                    last_touch_pos = curr;
                } else if t.phase == TouchPhase::Ended {
                    touch_drag_id = None;
                }
            }
            // -- Pinch to zoom
            if touches.len() == 2 {
                let t0 = &touches[0];
                let t1 = &touches[1];
                let pinch_distance = (t0.position - t1.position).length();
                if pinch_start.is_none() {
                    pinch_start = Some((t0.id, t1.id));
                    last_pinch_distance = pinch_distance;
                } else {
                    let diff = pinch_distance - last_pinch_distance;
                    camera.zoom *= 1.0 + (diff) * 0.002; // scale factor tweak
                    camera.zoom = camera.zoom.clamp(0.5, 3.0);
                    last_pinch_distance = pinch_distance;
                }
            } else {
                pinch_start = None;
            }
        }

        // -------- Zoom: Mouse wheel (Desktop only)
        if !mobile {
            camera.zoom *= 1.0 + mouse_wheel().1 * 0.05;
            camera.zoom = camera.zoom.clamp(0.5, 3.0);
        }

        // -------- Tile selection: Mouse or Touch Tap
        let select_tile = if !mobile {
            is_mouse_button_pressed(MouseButton::Right)
        } else {
            // Touch tap: one touch ended rapidly
            touches.iter().any(|t| t.phase == TouchPhase::Ended)
        };
        if select_tile {
            let (mx, my) = if !mobile {
                mouse_position()
            } else if let Some(t) = touches.last() {
                (t.position.x, t.position.y)
            } else {
                (0.0, 0.0)
            };
            let wx: f32 = (mx / camera.zoom - camera.offset.x) / TILE_SIZE;
            let wy: f32 = (my / camera.zoom - camera.offset.y) / TILE_SIZE;
            if wx >= 0.0 && wy >= 0.0 {
                let gx = wx.floor() as usize;
                let gy = wy.floor() as usize;
                if gx < GRID_SIZE && gy < GRID_SIZE {
                    selected = Some(Position { x: gx, y: gy });
                }
            }
        }

        set_camera(&Camera2D {
            zoom: vec2(
                2. / screen_width() * camera.zoom,
                2. / screen_height() * camera.zoom,
            ),
            target: Vec2::ZERO,
            offset: camera.offset,
            ..Default::default()
        });

        // -- Draw grid/tiles
        for (entity, pos) in world.positions.iter() {
            let r = world.renderables.get(entity).unwrap();
            let x = pos.x as f32 * TILE_SIZE;
            let y = pos.y as f32 * TILE_SIZE;
            draw_rectangle(x, y, TILE_SIZE - 1.0, TILE_SIZE - 1.0, r.color);
        }

        // -- Draw selection
        if let Some(sel) = selected {
            let x = sel.x as f32 * TILE_SIZE;
            let y = sel.y as f32 * TILE_SIZE;
            draw_rectangle_lines(x, y, TILE_SIZE, TILE_SIZE, 3.0, RED);
        }

        set_default_camera();

        // -- Minimal debug info
        draw_text(
            &format!(
                "Camera: ({:.1}, {:.1}) zoom {:.2}",
                camera.offset.x, camera.offset.y, camera.zoom
            ),
            10.0,
            30.0,
            24.0,
            DARKGRAY,
        );
        if let Some(sel) = selected {
            draw_text(
                &format!("Selected tile: ({},{})", sel.x, sel.y),
                10.0,
                60.0,
                24.0,
                ORANGE,
            );
        }
        draw_text(
            "L-drag: pan | Scroll/pinch: zoom | R-click/tap: select tile",
            10.0,
            screen_height() - 25.0,
            22.0,
            GRAY,
        );

        // Debug: show last touch delta on screen for mobile testing
        draw_text(
            &format!(
                "Touch delta: x={:.2} y={:.2}",
                last_touch_delta.x, last_touch_delta.y
            ),
            screen_width() - 220.0,
            30.0,
            20.0,
            DARKGRAY,
        );

        next_frame().await;
    }
}

// ...rest unchanged...
