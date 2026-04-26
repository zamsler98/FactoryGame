use crate::render_grid;
use crate::screens::{ScreenCommand, ScreenId};
use macroquad::prelude::*;
use std::collections::HashMap;

use game_logic::{update_world, InputFrame};

const TAP_MAX_MOVEMENT: f32 = 10.0;
const ROTATE_DEBOUNCE: f64 = 0.15;

pub struct GameplayScreen {
    world: game_core::World,
    camera: Camera2D,
    zoom: f32,
    prev_touches: HashMap<u64, Vec2>,
    touch_start: HashMap<u64, Vec2>,
    prev_mouse: Option<Vec2>,
    selected_spec_id: u32,
    selected_rotation: game_core::Rotation,
    last_rotate_time: f64,
    pointer: Option<(f32, f32)>,
    hover_tile: Option<game_core::TilePos>,
}

impl GameplayScreen {
    pub fn new() -> Self {
        let mut world = game_core::World::new();
        world.spawn_player(200.0, 200.0);
        world.spawn_enemy(500.0, 200.0);
        world.spawn_enemy(500.0, 400.0);

        Self {
            world,
            camera: Camera2D {
                target: vec2(0.0, 0.0),
                zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
                ..Default::default()
            },
            zoom: 1.0,
            prev_touches: HashMap::new(),
            touch_start: HashMap::new(),
            prev_mouse: None,
            selected_spec_id: 1,
            selected_rotation: game_core::Rotation::R0,
            last_rotate_time: 0.0,
            pointer: None,
            hover_tile: None,
        }
    }

    pub fn update(&mut self, dt: f32) -> ScreenCommand {
        if is_key_pressed(KeyCode::Escape) {
            return ScreenCommand::Switch(ScreenId::Home);
        }

        let mut input = InputFrame::default();

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            input.move_x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            input.move_x += 1.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            input.move_y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            input.move_y += 1.0;
        }

        let magnitude = (input.move_x * input.move_x + input.move_y * input.move_y).sqrt();
        if magnitude > 1.0 {
            input.move_x /= magnitude;
            input.move_y /= magnitude;
        }

        let touches_now = touches();
        let mut touch_pointer = None;
        input.action = is_key_pressed(KeyCode::Space)
            || (touches_now.is_empty() && is_mouse_button_pressed(MouseButton::Left));

        for touch in &touches_now {
            let position = touch.position;
            touch_pointer = Some(position);
            self.touch_start.entry(touch.id).or_insert(position);
        }

        if touches_now.len() == 1 {
            let touch = &touches_now[0];
            if let Some(last) = self.prev_touches.get(&touch.id) {
                let delta = touch.position - *last;
                self.camera.target -= vec2(delta.x, -delta.y) / self.zoom;
            }
        }

        if touches_now.is_empty() {
            let mouse_pos = vec2(mouse_position().0, mouse_position().1);
            if is_mouse_button_down(MouseButton::Left) {
                if let Some(prev) = self.prev_mouse {
                    let delta = mouse_pos - prev;
                    self.camera.target -= vec2(delta.x, -delta.y) / self.zoom;
                }
                self.prev_mouse = Some(mouse_pos);
            } else {
                self.prev_mouse = None;
            }
        }

        let current_ids: HashMap<u64, ()> =
            touches_now.iter().map(|touch| (touch.id, ())).collect();
        let ended_ids: Vec<u64> = self
            .touch_start
            .keys()
            .filter(|id| !current_ids.contains_key(id))
            .copied()
            .collect();

        for id in ended_ids {
            if let Some(start_pos) = self.touch_start.get(&id) {
                let end_pos = self.prev_touches.get(&id).copied().unwrap_or(*start_pos);
                if start_pos.distance(end_pos) < TAP_MAX_MOVEMENT {
                    input.action = true;
                    input.pointer = Some((end_pos.x, end_pos.y));
                }
            }

            self.touch_start.remove(&id);
            self.prev_touches.remove(&id);
        }

        self.prev_touches.clear();
        for touch in &touches_now {
            self.prev_touches.insert(touch.id, touch.position);
        }

        if input.pointer.is_none() {
            if let Some(pointer) = touch_pointer {
                input.pointer = Some((pointer.x, pointer.y));
            } else if is_mouse_button_down(MouseButton::Left) {
                input.pointer = Some(mouse_position());
            }
        }

        let hud = self.hud_layout();
        let mut hud_consumed = false;

        let mouse_clicked = touches_now.is_empty() && is_mouse_button_released(MouseButton::Left);
        if mouse_clicked {
            let point = mouse_position();
            if let Some(hud_target) = hud.button_at(point) {
                self.handle_hud_button(hud_target);
                hud_consumed = true;
            } else {
                input.action = true;
                input.pointer = Some(point);
            }
        }

        let hover_tile = input.pointer.map(|(x, y)| {
            let world = self.screen_to_world_vec2(vec2(x, y));
            game_core::TilePos {
                x: (world.x / render_grid::TILE_PX).floor() as i32,
                y: (world.y / render_grid::TILE_PX).floor() as i32,
            }
        });

        update_world(&mut self.world, &input, dt);

        if input.action && !hud_consumed {
            if let Some(point) = input.pointer {
                if let Some(hud_target) = hud.button_at(point) {
                    self.handle_hud_button(hud_target);
                } else if let Some(tile) = hover_tile {
                    let spec = game_core::BuildingSpec {
                        spec_id: self.selected_spec_id,
                        size: game_core::Size2 { w: 1, h: 1 },
                    };
                    let _ = game_logic::placement::try_place_building(
                        &mut self.world.tile_grid,
                        &spec,
                        tile,
                        self.selected_rotation,
                    );
                }
            }
        }

        self.pointer = input.pointer;
        self.hover_tile = hover_tile;

        ScreenCommand::None
    }

    pub fn draw(&mut self) {
        let grid_snapshot = game_logic::placement::grid_snapshot(&self.world.tile_grid);
        let top_left_world = self.screen_to_world_vec2(vec2(0.0, 0.0));
        let bottom_right_world = self.screen_to_world_vec2(vec2(screen_width(), screen_height()));
        let world_min_x = top_left_world.x.min(bottom_right_world.x);
        let world_max_x = top_left_world.x.max(bottom_right_world.x);
        let world_min_y = top_left_world.y.min(bottom_right_world.y);
        let world_max_y = top_left_world.y.max(bottom_right_world.y);
        let min_x = (world_min_x / render_grid::TILE_PX).floor() as i32;
        let min_y = (world_min_y / render_grid::TILE_PX).floor() as i32;
        let max_x = (world_max_x / render_grid::TILE_PX).floor() as i32;
        let max_y = (world_max_y / render_grid::TILE_PX).floor() as i32;

        self.camera.zoom = vec2(
            self.zoom / screen_width() * 2.0,
            -self.zoom / screen_height() * 2.0,
        );

        set_camera(&self.camera);
        render_grid::draw_grid(&grid_snapshot, self.hover_tile, min_x, max_x, min_y, max_y);
        set_default_camera();

        let hud = self.hud_layout();
        draw_rectangle(
            hud.btn1_x,
            hud.base_y,
            hud.btn_size,
            hud.btn_size,
            Color::new(1.0, 1.0, 1.0, 0.95),
        );
        draw_rectangle(
            hud.btn2_x,
            hud.base_y,
            hud.btn_size,
            hud.btn_size,
            Color::new(0.9, 0.6, 0.3, 0.95),
        );
        draw_rectangle(
            hud.btn3_x,
            hud.base_y,
            hud.btn_size,
            hud.btn_size,
            Color::new(0.3, 0.8, 0.4, 0.95),
        );
        draw_rectangle(
            hud.rotate_x,
            hud.base_y,
            hud.btn_size,
            hud.btn_size,
            Color::new(0.2, 0.2, 0.2, 0.95),
        );

        let selected_x = match self.selected_spec_id {
            1 => hud.btn1_x,
            2 => hud.btn2_x,
            3 => hud.btn3_x,
            _ => hud.btn1_x,
        };
        draw_rectangle_lines(
            selected_x,
            hud.base_y,
            hud.btn_size,
            hud.btn_size,
            4.0,
            Color::new(1.0, 1.0, 0.0, 0.95),
        );

        let rotation_label = match self.selected_rotation {
            game_core::Rotation::R0 => "0",
            game_core::Rotation::R90 => "90",
            game_core::Rotation::R180 => "180",
            game_core::Rotation::R270 => "270",
        };
        draw_text(
            rotation_label,
            hud.rotate_x + 12.0,
            hud.base_y + hud.btn_size / 2.0 + 6.0,
            22.0,
            WHITE,
        );

        let selected_name = match self.selected_spec_id {
            1 => "Conveyor",
            2 => "Miner",
            3 => "Smelter",
            _ => "Unknown",
        };
        draw_text(selected_name, hud.hud_margin, hud.base_y - 8.0, 20.0, WHITE);

        if let Some((x, y)) = self.pointer {
            draw_circle(x, y, 6.0, Color::new(1.0, 1.0, 0.0, 1.0));
        }

        draw_text(
            "Tap a building, then place it on the grid. Press Esc to return home.",
            20.0,
            20.0,
            20.0,
            WHITE,
        );
    }

    fn handle_hud_button(&mut self, button: HudButton) {
        match button {
            HudButton::Conveyor => self.selected_spec_id = 1,
            HudButton::Miner => self.selected_spec_id = 2,
            HudButton::Smelter => self.selected_spec_id = 3,
            HudButton::Rotate => self.cycle_rotation(),
        }
    }

    fn cycle_rotation(&mut self) {
        let current_time = get_time();
        if (current_time - self.last_rotate_time) <= ROTATE_DEBOUNCE {
            return;
        }

        self.selected_rotation = match self.selected_rotation {
            game_core::Rotation::R0 => game_core::Rotation::R90,
            game_core::Rotation::R90 => game_core::Rotation::R180,
            game_core::Rotation::R180 => game_core::Rotation::R270,
            game_core::Rotation::R270 => game_core::Rotation::R0,
        };
        self.last_rotate_time = current_time;
    }

    fn hud_layout(&self) -> HudLayout {
        let hud_margin = 16.0;
        let btn_size = 56.0;
        let spacing = 12.0;
        let base_y = screen_height() - hud_margin - btn_size;
        let btn1_x = hud_margin;
        let btn2_x = btn1_x + btn_size + spacing;
        let btn3_x = btn2_x + btn_size + spacing;
        let rotate_x = btn3_x + btn_size + spacing;

        HudLayout {
            hud_margin,
            btn_size,
            base_y,
            btn1_x,
            btn2_x,
            btn3_x,
            rotate_x,
        }
    }

    fn screen_to_world_vec2(&self, screen_pos: Vec2) -> Vec2 {
        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let relative = screen_pos - screen_center;
        vec2(
            self.camera.target.x + relative.x / self.zoom,
            self.camera.target.y - relative.y / self.zoom,
        )
    }
}

#[derive(Clone, Copy)]
enum HudButton {
    Conveyor,
    Miner,
    Smelter,
    Rotate,
}

struct HudLayout {
    hud_margin: f32,
    btn_size: f32,
    base_y: f32,
    btn1_x: f32,
    btn2_x: f32,
    btn3_x: f32,
    rotate_x: f32,
}

impl HudLayout {
    fn button_at(&self, point: (f32, f32)) -> Option<HudButton> {
        if point_in_rect(
            point,
            self.btn1_x,
            self.base_y,
            self.btn_size,
            self.btn_size,
        ) {
            return Some(HudButton::Conveyor);
        }
        if point_in_rect(
            point,
            self.btn2_x,
            self.base_y,
            self.btn_size,
            self.btn_size,
        ) {
            return Some(HudButton::Miner);
        }
        if point_in_rect(
            point,
            self.btn3_x,
            self.base_y,
            self.btn_size,
            self.btn_size,
        ) {
            return Some(HudButton::Smelter);
        }
        if point_in_rect(
            point,
            self.rotate_x,
            self.base_y,
            self.btn_size,
            self.btn_size,
        ) {
            return Some(HudButton::Rotate);
        }

        None
    }
}

fn point_in_rect(point: (f32, f32), x: f32, y: f32, w: f32, h: f32) -> bool {
    point.0 >= x && point.0 <= x + w && point.1 >= y && point.1 <= y + h
}
