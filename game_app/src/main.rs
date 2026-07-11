//! game_app: Macroquad application glue.
//! - captures platform input and fills `InputFrame`
//! - calls `game_logic::update_world`
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.

use game_core::{BuildingKind, Rotation, TilePos};
use game_logic::{update_world, InputFrame};
use macroquad::prelude::*;
use std::collections::HashMap;

mod render_grid;

const HUD_MARGIN: f32 = 16.0;
const BTN_SIZE: f32 = 56.0;
const BTN_SPACING: f32 = 12.0;
const ROTATE_DEBOUNCE: f64 = 0.15;
const TAP_MAX_MOVEMENT: f32 = 10.0;

#[derive(Clone, Copy)]
enum HudAction {
    Select(BuildingKind),
    Rotate,
}

/// Screen-space toolbar: one button per building kind plus a rotate button.
struct Toolbar {
    base_y: f32,
    slots: Vec<(f32, HudAction)>,
}

impl Toolbar {
    fn layout() -> Self {
        let base_y = screen_height() - HUD_MARGIN - BTN_SIZE;
        let mut slots = Vec::new();
        let mut x = HUD_MARGIN;
        for kind in BuildingKind::ALL {
            slots.push((x, HudAction::Select(kind)));
            x += BTN_SIZE + BTN_SPACING;
        }
        slots.push((x, HudAction::Rotate));
        Self { base_y, slots }
    }

    fn hit(&self, px: f32, py: f32) -> Option<HudAction> {
        if py < self.base_y || py > self.base_y + BTN_SIZE {
            return None;
        }
        self.slots
            .iter()
            .find(|(x, _)| px >= *x && px <= *x + BTN_SIZE)
            .map(|(_, action)| *action)
    }

    fn draw(&self, selected_kind: BuildingKind, selected_rotation: Rotation) {
        for (x, action) in &self.slots {
            match action {
                HudAction::Select(kind) => {
                    draw_rectangle(
                        *x,
                        self.base_y,
                        BTN_SIZE,
                        BTN_SIZE,
                        render_grid::building_color(Some(*kind)),
                    );
                    if *kind == selected_kind {
                        draw_rectangle_lines(
                            *x,
                            self.base_y,
                            BTN_SIZE,
                            BTN_SIZE,
                            4.0,
                            Color::new(1.0, 1.0, 0.0, 0.95),
                        );
                    }
                }
                HudAction::Rotate => {
                    draw_rectangle(
                        *x,
                        self.base_y,
                        BTN_SIZE,
                        BTN_SIZE,
                        Color::new(0.2, 0.2, 0.2, 0.95),
                    );
                    let rot_label = match selected_rotation {
                        Rotation::R0 => "0°",
                        Rotation::R90 => "90°",
                        Rotation::R180 => "180°",
                        Rotation::R270 => "270°",
                    };
                    draw_text(
                        rot_label,
                        *x + 8.0,
                        self.base_y + BTN_SIZE / 2.0 + 6.0,
                        20.0,
                        WHITE,
                    );
                }
            }
        }

        // Selected building name above the toolbar
        draw_text(
            selected_kind.name(),
            HUD_MARGIN,
            self.base_y - 8.0,
            20.0,
            WHITE,
        );
    }
}

#[macroquad::main("FactoryGame - Macroquad")]
async fn main() {
    let mut world = game_core::World::new();

    // Camera & zoom state for panning/zooming. Positive zoom.y renders
    // world +y downward on screen, so the tile grid (which lives in positive
    // coordinates) extends down-right and Rotation::R90 points down.
    let mut camera = Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
        ..Default::default()
    };
    let zoom: f32 = 1.0;

    // Touch tap detection state (for mobile taps -> action)
    let mut prev_touches: HashMap<u64, Vec2> = HashMap::new();
    let mut touch_start: HashMap<u64, Vec2> = HashMap::new();

    // Mouse drag state: previous position for panning, press position for
    // distinguishing clicks from drags.
    let mut prev_mouse: Option<Vec2> = None;
    let mut mouse_press: Option<Vec2> = None;

    // Selected building and rotation state (persist across frames)
    let mut selected_kind = BuildingKind::Conveyor;
    let mut selected_rotation = Rotation::R0;
    let mut last_rotate_time: f64 = 0.0;

    loop {
        let dt = get_frame_time();

        // Build a platform-agnostic InputFrame from Macroquad input APIs.
        let mut input = InputFrame::default();

        // Mobile touch: collect touches for pointer and tap detection (no joystick)
        let mut touch_pointer: Option<Vec2> = None;
        let touches_now = touches();

        // Action (space / mouse click). Ignore mouse events when touch input is present
        input.action = is_key_pressed(KeyCode::Space)
            || (touches_now.is_empty() && is_mouse_button_pressed(MouseButton::Left));

        // Record current touches, set pointer to first touch
        for t in &touches_now {
            let tp = t.position;
            touch_pointer = Some(tp);
            touch_start.entry(t.id).or_insert(tp);
        }

        // -----------------------------
        // Panning: single-finger drag and mouse drag.
        // Deltas are converted through the camera so screen motion maps 1:1
        // to world motion regardless of zoom or axis orientation.
        // -----------------------------
        if touches_now.len() == 1 {
            let t = &touches_now[0];
            if let Some(last) = prev_touches.get(&t.id) {
                let delta_world =
                    camera.screen_to_world(*last) - camera.screen_to_world(t.position);
                camera.target += delta_world;
            }
        }

        if touches_now.len() == 2 {
            // optional: could implement pinch-to-zoom here later
        }

        // Mouse-drag panning (desktop)
        let mut mouse_clicked = false;
        if touches_now.is_empty() {
            let mouse_pos = vec2(mouse_position().0, mouse_position().1);
            if is_mouse_button_pressed(MouseButton::Left) {
                mouse_press = Some(mouse_pos);
            }
            if is_mouse_button_down(MouseButton::Left) {
                if let Some(prev) = prev_mouse {
                    let delta_world =
                        camera.screen_to_world(prev) - camera.screen_to_world(mouse_pos);
                    camera.target += delta_world;
                }
                prev_mouse = Some(mouse_pos);
            } else {
                prev_mouse = None;
            }
            // A release counts as a click only if the mouse barely moved
            // since the press (otherwise it was a pan).
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(press) = mouse_press.take() {
                    mouse_clicked = press.distance(mouse_pos) < TAP_MAX_MOVEMENT;
                }
            }
        }

        // Detect ended touches as taps (small movement)
        let mut tap_pos: Option<Vec2> = None;
        {
            let mut current_ids: HashMap<u64, ()> = HashMap::new();
            for t in &touches_now {
                current_ids.insert(t.id, ());
            }

            let ended_ids: Vec<u64> = touch_start
                .keys()
                .filter(|id| !current_ids.contains_key(id))
                .copied()
                .collect();

            for id in ended_ids {
                if let Some(start_pos) = touch_start.get(&id) {
                    let end_pos = prev_touches.get(&id).cloned().unwrap_or(*start_pos);
                    if start_pos.distance(end_pos) < TAP_MAX_MOVEMENT {
                        // treat as tap
                        input.action = true;
                        input.pointer = Some((end_pos.x, end_pos.y));
                        tap_pos = Some(end_pos);
                    }
                }
                touch_start.remove(&id);
                prev_touches.remove(&id);
            }
        }

        // Update prev_touches to current touches for the next frame
        prev_touches.clear();
        for t in &touches_now {
            prev_touches.insert(t.id, t.position);
        }

        // Pointer / touch: prefer first touch if present, otherwise mouse.
        if input.pointer.is_none() {
            if let Some(tp) = touch_pointer {
                input.pointer = Some((tp.x, tp.y));
            } else {
                input.pointer = Some(mouse_position());
            }
        }

        // Hovered tile from the pointer, via the camera's own inverse transform.
        let hover_tile = input.pointer.map(|(sx, sy)| {
            let world = camera.screen_to_world(vec2(sx, sy));
            TilePos {
                x: (world.x / render_grid::TILE_PX).floor() as i32,
                y: (world.y / render_grid::TILE_PX).floor() as i32,
            }
        });

        // Update game state using platform-agnostic logic
        update_world(&mut world, &input, dt);

        // -----------------------------
        // HUD clicks and building placement. A confirmed click/tap either
        // hits a toolbar button or places the selected building on the grid.
        // -----------------------------
        let toolbar = Toolbar::layout();

        // Confirmed clicks only: a mouse release that didn't pan, a touch
        // tap, or the space key at the current pointer. A bare mouse *press*
        // must not count — it may be the start of a pan drag.
        let click: Option<Vec2> = if mouse_clicked {
            let (px, py) = mouse_position();
            Some(vec2(px, py))
        } else if tap_pos.is_some() {
            tap_pos
        } else if is_key_pressed(KeyCode::Space) {
            input.pointer.map(|(px, py)| vec2(px, py))
        } else {
            None
        };

        if let Some(pos) = click {
            match toolbar.hit(pos.x, pos.y) {
                Some(HudAction::Select(kind)) => selected_kind = kind,
                Some(HudAction::Rotate) => {
                    // debounce to avoid double-fire from multiple input paths
                    let now = get_time();
                    if (now - last_rotate_time) > ROTATE_DEBOUNCE {
                        selected_rotation = selected_rotation.rotated_cw();
                        last_rotate_time = now;
                    }
                }
                None => {
                    let world_pos = camera.screen_to_world(pos);
                    let tile = TilePos {
                        x: (world_pos.x / render_grid::TILE_PX).floor() as i32,
                        y: (world_pos.y / render_grid::TILE_PX).floor() as i32,
                    };
                    let _ = game_logic::placement::try_place_building(
                        &mut world,
                        selected_kind,
                        tile,
                        selected_rotation,
                    );
                }
            }
        }

        // --- Rendering (platform-specific) ---
        let grid_snapshot = game_logic::placement::grid_snapshot(&world.tile_grid);
        let factory = game_logic::view::factory_snapshot(&world);

        // visible bounds in tile coordinates
        let top_left_world = camera.screen_to_world(vec2(0.0, 0.0));
        let bottom_right_world = camera.screen_to_world(vec2(screen_width(), screen_height()));
        let world_min_x = top_left_world.x.min(bottom_right_world.x);
        let world_max_x = top_left_world.x.max(bottom_right_world.x);
        let world_min_y = top_left_world.y.min(bottom_right_world.y);
        let world_max_y = top_left_world.y.max(bottom_right_world.y);
        let min_x = (world_min_x / render_grid::TILE_PX).floor() as i32;
        let min_y = (world_min_y / render_grid::TILE_PX).floor() as i32;
        let max_x = (world_max_x / render_grid::TILE_PX).floor() as i32;
        let max_y = (world_max_y / render_grid::TILE_PX).floor() as i32;

        // Apply camera and draw the world: grid, buildings, then items on top
        set_camera(&camera);
        render_grid::draw_grid(&grid_snapshot, hover_tile, min_x, max_x, min_y, max_y);
        render_grid::draw_items(&factory.items);
        render_grid::draw_machine_overlays(&factory.machines);
        set_default_camera();

        // Keep camera zoom in sync with window size
        camera.zoom = vec2(zoom / screen_width() * 2.0, zoom / screen_height() * 2.0);

        // --- HUD draw (screen-space)
        toolbar.draw(selected_kind, selected_rotation);

        draw_text(
            "Tap button to select building, tap grid to place | drag: pan",
            20.0,
            20.0,
            20.0,
            WHITE,
        );

        // Production tallies
        let mut text_y = 44.0;
        for (kind, count) in &factory.produced {
            draw_text(
                &format!("{}: {}", kind.name(), count),
                20.0,
                text_y,
                20.0,
                WHITE,
            );
            text_y += 22.0;
        }

        next_frame().await;
    }
}
