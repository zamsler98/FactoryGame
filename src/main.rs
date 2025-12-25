//! Main entry point for FactoryGame with custom ECS, grid world, and interactive grid rendering.
use macroquad::prelude::*;
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
const TILE_SIZE: f32 = 64.0;

const PAN_SPEED: f32 = 0.01;

struct Camera {
    offset: Vec2,
    zoom: f32,
}

#[macroquad::main("FactoryGame")]
async fn main() {
    let mut world = World::new();
    // Simple grid of ground/resource tiles (use entities for now)
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

    loop {
        clear_background(LIGHTGRAY);

        // -- Handle camera pan (drag background to move)
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
        // -- Zoom with wheel/pinch
        camera.zoom *= 1.0 + mouse_wheel().1 * 0.05;
        camera.zoom = camera.zoom.clamp(0.5, 3.0);

        // -- Tap/click to select tile
        if is_mouse_button_pressed(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let wx = (mx / camera.zoom - camera.offset.x) / TILE_SIZE;
            let wy = (my / camera.zoom - camera.offset.y) / TILE_SIZE;
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
            "L-drag: pan | Scroll: zoom | R-click: select tile",
            10.0,
            screen_height() - 25.0,
            22.0,
            GRAY,
        );

        next_frame().await;
    }
}
