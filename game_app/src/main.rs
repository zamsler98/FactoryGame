//! game_app: Macroquad application glue.
//! - captures platform input
//! - drives building placement / mining / crafting through `game_logic`
//! - renders the world and the HUD
//!
//! Only this crate depends on `macroquad`.

use game_core::{
    assembler_recipes, hand_recipes, recipe, BuildingKind, BuildingState, InstanceId, ItemKind,
    RecipeId, Rotation, TilePos, World,
};
use game_logic::{update_world, InputFrame};
use macroquad::prelude::*;

mod render_grid;
use render_grid::TILE_PX;

const PANEL_W: f32 = 250.0;
const ROW_H: f32 = 22.0;
const BTN_SIZE: f32 = 52.0;
const HUD_MARGIN: f32 = 12.0;
const TAP_MAX_MOVEMENT: f32 = 8.0;

/// A rectangle hit test.
fn inside(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    px >= x && px <= x + w && py >= y && py <= y + h
}

/// Draw a labelled button; returns true when `click` landed inside it.
fn button(x: f32, y: f32, w: f32, h: f32, label: &str, enabled: bool, click: Option<Vec2>) -> bool {
    let bg = if enabled {
        Color::new(0.22, 0.24, 0.30, 0.95)
    } else {
        Color::new(0.15, 0.15, 0.17, 0.9)
    };
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(0.5, 0.5, 0.55, 0.8));
    let col = if enabled { WHITE } else { GRAY };
    draw_text(label, x + 6.0, y + h * 0.5 + 5.0, 16.0, col);
    enabled && click.is_some_and(|c| inside(c.x, c.y, x, y, w, h))
}

fn window_conf() -> Conf {
    Conf {
        window_title: "FactoryGame".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();

    // Center the camera roughly over the starting ore field.
    let mut camera = Camera2D {
        target: vec2(16.0 * TILE_PX, 14.0 * TILE_PX),
        zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
        ..Default::default()
    };
    let mut zoom: f32 = 1.0;

    let mut prev_mouse: Option<Vec2> = None;
    let mut mouse_press: Option<Vec2> = None;
    // Finger separation last frame, for two-finger pinch-to-zoom.
    let mut prev_pinch: Option<f32> = None;

    let mut selected_kind = BuildingKind::Belt;
    let mut selected_rotation = Rotation::R0;
    let mut selected_inst: Option<InstanceId> = None;
    // Mine mode replaces right-click on touch: when on, a tap removes a building.
    let mut mine_mode = false;
    // The right-hand inventory/crafting panel can be hidden to free screen space.
    let mut show_panel = true;

    // Recipe options an assembler can cycle through: None + each crafting recipe.
    let mut asm_options: Vec<Option<RecipeId>> = vec![None];
    asm_options.extend(assembler_recipes().map(|r| Some(r.id)));

    loop {
        let dt = get_frame_time();
        let sw = screen_width();
        let sh = screen_height();
        let panel_x = if show_panel { sw - PANEL_W } else { sw };
        // Size the toolbar buttons to fit the available width (below the panel),
        // so all buildings + Rotate + Mine stay on screen on narrow phones.
        let n_tools = (BuildingKind::ALL.len() + 2) as f32;
        let tb_gap = 6.0;
        // The toolbar runs the full width along the bottom; the side panel stops
        // above it (see below) so no building button hides behind the panel.
        let tb_avail = sw - 2.0 * HUD_MARGIN;
        let btn = ((tb_avail - tb_gap * (n_tools - 1.0)) / n_tools).clamp(34.0, BTN_SIZE);
        let toolbar_y = sh - HUD_MARGIN - btn;
        let mut input = InputFrame::default();

        // `left_click` is a discrete "act here" tap in screen space. `pointer`
        // tracks the cursor / finger for the placement preview. On the web,
        // miniquad synthesizes mouse events from single-finger touches, so the
        // same mouse path drives both desktop and mobile; `touches()` is used
        // only for the two-finger pinch that a mouse cannot express.
        let mut left_click: Option<Vec2> = None;
        let pointer = vec2(mouse_position().0, mouse_position().1);

        // ---- Two-finger pinch-to-zoom (mobile) ----
        let active = touches();
        let pinching = active.len() >= 2;
        if pinching {
            let dist = active[0].position.distance(active[1].position);
            if let Some(prev) = prev_pinch {
                if prev > 1.0 {
                    zoom = (zoom * (dist / prev)).clamp(0.3, 4.0);
                }
            }
            prev_pinch = Some(dist);
        } else {
            prev_pinch = None;
        }

        // ---- Zoom (mouse wheel) + keyboard select/rotate/pan (desktop) ----
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            zoom = (zoom * if wheel_y > 0.0 { 1.1 } else { 1.0 / 1.1 }).clamp(0.3, 4.0);
        }
        let keys = [
            (KeyCode::Key1, BuildingKind::Belt),
            (KeyCode::Key2, BuildingKind::Miner),
            (KeyCode::Key3, BuildingKind::Furnace),
            (KeyCode::Key4, BuildingKind::Inserter),
            (KeyCode::Key5, BuildingKind::Assembler),
            (KeyCode::Key6, BuildingKind::Chest),
        ];
        for (k, kind) in keys {
            if is_key_pressed(k) {
                selected_kind = kind;
                mine_mode = false;
            }
        }
        if is_key_pressed(KeyCode::R) {
            selected_rotation = selected_rotation.rotated_cw();
        }
        let pan_speed = 400.0 * dt / zoom;
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            camera.target.x -= pan_speed;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            camera.target.x += pan_speed;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            camera.target.y -= pan_speed;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            camera.target.y += pan_speed;
        }

        // ---- Drag pans, click/tap (no drag) acts, right-click mines ----
        // During a pinch we ignore the synthesized mouse so the map doesn't
        // jump; we also drop any pending tap so the pinch never places.
        if is_mouse_button_pressed(MouseButton::Left) {
            mouse_press = Some(pointer);
        }
        if pinching {
            prev_mouse = None;
            mouse_press = None;
        } else if is_mouse_button_down(MouseButton::Left) {
            if let Some(prev) = prev_mouse {
                let delta = camera.screen_to_world(prev) - camera.screen_to_world(pointer);
                camera.target += delta;
            }
            prev_mouse = Some(pointer);
        } else {
            prev_mouse = None;
        }
        if is_mouse_button_released(MouseButton::Left) {
            if let Some(press) = mouse_press.take() {
                if press.distance(pointer) < TAP_MAX_MOVEMENT {
                    left_click = Some(pointer);
                }
            }
        }
        let right_click = is_mouse_button_pressed(MouseButton::Right);

        camera.zoom = vec2(2.0 * zoom / sw, 2.0 * zoom / sh);
        input.pointer = Some((pointer.x, pointer.y));

        // Tile under the cursor / last finger (drives the placement preview).
        let hover_tile = {
            let w = camera.screen_to_world(pointer);
            TilePos {
                x: (w.x / TILE_PX).floor() as i32,
                y: (w.y / TILE_PX).floor() as i32,
            }
        };

        // ---- Advance simulation ----
        update_world(&mut world, &input, dt);

        // ================= Render world =================
        let grid_snapshot = game_logic::placement::grid_snapshot(&world.tile_grid);
        let resources = game_logic::placement::resource_snapshot(&world);
        let factory = game_logic::view::factory_snapshot(&world);

        let tl = camera.screen_to_world(vec2(0.0, 0.0));
        let br = camera.screen_to_world(vec2(sw, sh));
        let min_x = (tl.x.min(br.x) / TILE_PX).floor() as i32;
        let min_y = (tl.y.min(br.y) / TILE_PX).floor() as i32;
        let max_x = (tl.x.max(br.x) / TILE_PX).floor() as i32;
        let max_y = (tl.y.max(br.y) / TILE_PX).floor() as i32;
        let hover_ok = can_place_here(&world, selected_kind, hover_tile, selected_rotation);

        clear_background(Color::from_rgba(20, 20, 24, 255));
        set_camera(&camera);
        render_grid::draw_resources(&resources, min_x, max_x, min_y, max_y);
        render_grid::draw_grid(
            &grid_snapshot,
            Some(hover_tile),
            hover_ok,
            min_x,
            max_x,
            min_y,
            max_y,
        );
        render_grid::draw_items(&factory.items);
        render_grid::draw_machine_overlays(&factory.machines);
        set_default_camera();

        // ================= HUD overlay (top-left) =================
        draw_hud_overlay(&factory, hover_tile);

        // ================= UI panels (screen space) =================
        // Track whether a click was consumed by the UI so it does not also
        // place/select a building in the world.
        let mut ui_consumed = false;

        // ---- Bottom toolbar: building palette + Rotate + Mine ----
        {
            let mut x = HUD_MARGIN;
            for kind in BuildingKind::ALL {
                let count = world.inventory.count(kind.item());
                draw_rectangle(
                    x,
                    toolbar_y,
                    btn,
                    btn,
                    render_grid::building_color(Some(kind)),
                );
                if kind == selected_kind && !mine_mode {
                    draw_rectangle_lines(
                        x,
                        toolbar_y,
                        btn,
                        btn,
                        3.0,
                        Color::new(1.0, 1.0, 0.0, 0.95),
                    );
                }
                draw_text(
                    &format!("{count}"),
                    x + 4.0,
                    toolbar_y + btn - 6.0,
                    18.0,
                    BLACK,
                );
                if let Some(c) = left_click {
                    if inside(c.x, c.y, x, toolbar_y, btn, btn) {
                        selected_kind = kind;
                        mine_mode = false;
                        ui_consumed = true;
                    }
                }
                x += btn + tb_gap;
            }
            if button(
                x,
                toolbar_y,
                btn,
                btn,
                rot_label(selected_rotation),
                true,
                left_click,
            ) {
                selected_rotation = selected_rotation.rotated_cw();
                ui_consumed = true;
            }
            x += btn + tb_gap;
            // Mine toggle (replaces right-click on touch devices).
            if button(x, toolbar_y, btn, btn, "Mine", true, left_click) {
                mine_mode = !mine_mode;
                ui_consumed = true;
            }
            if mine_mode {
                draw_rectangle_lines(x, toolbar_y, btn, btn, 3.0, Color::new(1.0, 0.3, 0.3, 0.95));
            }
            let status = if mine_mode {
                "Mode: MINE (tap a building to remove)".to_string()
            } else {
                format!("Place: {}", selected_kind.name())
            };
            draw_text(&status, HUD_MARGIN, toolbar_y - 8.0, 18.0, WHITE);
        }

        // ---- Panel toggle (top-right corner, always visible) ----
        {
            let bw = 40.0;
            let bx = sw - bw - HUD_MARGIN;
            let by = HUD_MARGIN;
            let label = if show_panel { ">>" } else { "<<" };
            if button(bx, by, bw, 28.0, label, true, left_click) {
                show_panel = !show_panel;
                ui_consumed = true;
            }
        }

        // ---- Right panel: inventory / crafting / inspector ----
        // Stops above the full-width toolbar so it never covers the buttons.
        draw_rectangle(
            panel_x,
            0.0,
            PANEL_W,
            toolbar_y,
            Color::new(0.08, 0.08, 0.10, 0.92),
        );
        let mut y = 10.0;
        draw_text("INVENTORY", panel_x + 10.0, y + 12.0, 20.0, WHITE);
        y += ROW_H;
        for (item, n) in world.inventory.stacks() {
            draw_rectangle(panel_x + 10.0, y, 14.0, 14.0, render_grid::item_color(item));
            draw_text(
                &format!("{}  x{}", item.name(), n),
                panel_x + 30.0,
                y + 12.0,
                16.0,
                WHITE,
            );
            y += ROW_H;
        }

        // Crafting queue
        y += 8.0;
        draw_text("HAND CRAFTING", panel_x + 10.0, y + 12.0, 20.0, WHITE);
        y += ROW_H;
        if let Some(active) = world.crafting.active() {
            let name = recipe(active.recipe).map(|r| r.name).unwrap_or("?");
            draw_text(
                &format!("Crafting {} {:.0}%", name, active.progress * 100.0),
                panel_x + 10.0,
                y + 12.0,
                15.0,
                YELLOW,
            );
        } else {
            draw_text("Crafting: idle", panel_x + 10.0, y + 12.0, 15.0, GRAY);
        }
        let pending = world.crafting.pending().len();
        if pending > 0 {
            draw_text(
                &format!("(+{pending})"),
                panel_x + 190.0,
                y + 12.0,
                15.0,
                GRAY,
            );
        }
        y += ROW_H;
        for r in hand_recipes() {
            let can = r
                .inputs
                .iter()
                .all(|i| world.inventory.count(i.item) >= i.count);
            let label = format!("{}  ({})", r.name, ingredients_label(r));
            if button(
                panel_x + 10.0,
                y,
                PANEL_W - 20.0,
                ROW_H - 3.0,
                &label,
                true,
                left_click,
            ) {
                world.queue_craft(r.id);
                ui_consumed = true;
            }
            if !can {
                draw_rectangle(
                    panel_x + 10.0,
                    y,
                    PANEL_W - 20.0,
                    ROW_H - 3.0,
                    Color::new(0.0, 0.0, 0.0, 0.4),
                );
            }
            y += ROW_H;
        }

        // ---- Inspector for the selected building ----
        y += 8.0;
        if let Some(id) = selected_inst {
            if let Some(info) = inspector_info(&world, id) {
                draw_text("SELECTED", panel_x + 10.0, y + 12.0, 20.0, WHITE);
                y += ROW_H;
                draw_text(&info.title, panel_x + 10.0, y + 12.0, 16.0, SKYBLUE);
                y += ROW_H;
                for line in &info.lines {
                    draw_text(line, panel_x + 10.0, y + 12.0, 15.0, LIGHTGRAY);
                    y += ROW_H - 4.0;
                }
                y += 4.0;
                match info.kind {
                    BuildingKind::Assembler => {
                        let cur = asm_options
                            .iter()
                            .position(|o| *o == info.assembler_recipe)
                            .unwrap_or(0);
                        let n = asm_options.len();
                        if button(panel_x + 10.0, y, 60.0, ROW_H, "< prev", true, left_click) {
                            world.set_assembler_recipe(id, asm_options[(cur + n - 1) % n]);
                            ui_consumed = true;
                        }
                        if button(panel_x + 80.0, y, 60.0, ROW_H, "next >", true, left_click) {
                            world.set_assembler_recipe(id, asm_options[(cur + 1) % n]);
                            ui_consumed = true;
                        }
                    }
                    BuildingKind::Chest => {
                        let clicked = button(
                            panel_x + 10.0,
                            y,
                            120.0,
                            ROW_H,
                            "Take all",
                            true,
                            left_click,
                        );
                        if clicked {
                            world.take_all_from_chest(id);
                            ui_consumed = true;
                        }
                    }
                    BuildingKind::Miner | BuildingKind::Furnace => {
                        let has_coal = world.inventory.count(ItemKind::Coal) > 0;
                        if button(
                            panel_x + 10.0,
                            y,
                            150.0,
                            ROW_H,
                            "Add coal fuel",
                            has_coal,
                            left_click,
                        ) {
                            world.add_fuel_from_inventory(id);
                            ui_consumed = true;
                        }
                    }
                    _ => {}
                }
            } else {
                selected_inst = None;
            }
        }

        // Any click on the right panel or the toolbar strip is UI, not world.
        if let Some(c) = left_click {
            if c.x >= panel_x || c.y >= toolbar_y {
                ui_consumed = true;
            }
        }

        // ================= World interaction =================
        // A world tap mines (in Mine mode), else selects an existing building or
        // places the selected one. Desktop right-click always mines.
        if let Some(c) = left_click {
            if !ui_consumed {
                let w = camera.screen_to_world(c);
                let tile = TilePos {
                    x: (w.x / TILE_PX).floor() as i32,
                    y: (w.y / TILE_PX).floor() as i32,
                };
                if mine_mode {
                    if world.tile_grid.tile_occupant(tile) == selected_inst {
                        selected_inst = None;
                    }
                    game_logic::placement::mine_at(&mut world, tile);
                } else if let Some(id) = world.tile_grid.tile_occupant(tile) {
                    selected_inst = Some(id);
                } else {
                    let _ = game_logic::placement::try_place_building(
                        &mut world,
                        selected_kind,
                        tile,
                        selected_rotation,
                    );
                }
            }
        }
        if right_click && pointer.x < panel_x && pointer.y < toolbar_y {
            if world.tile_grid.tile_occupant(hover_tile) == selected_inst {
                selected_inst = None;
            }
            game_logic::placement::mine_at(&mut world, hover_tile);
        }

        next_frame().await;
    }
}

fn rot_label(r: Rotation) -> &'static str {
    match r {
        Rotation::R0 => "0",
        Rotation::R90 => "90",
        Rotation::R180 => "180",
        Rotation::R270 => "270",
    }
}

fn ingredients_label(r: &game_core::Recipe) -> String {
    r.inputs
        .iter()
        .map(|i| format!("{}x{}", i.count, short(i.item.name())))
        .collect::<Vec<_>>()
        .join("+")
}

fn can_place_here(world: &World, kind: BuildingKind, tile: TilePos, rot: Rotation) -> bool {
    if world.inventory.count(kind.item()) == 0 {
        return false;
    }
    if kind.requires_resource() && world.resources.get(tile).is_none() {
        return false;
    }
    world.tile_grid.can_place(&kind.spec(), tile, rot)
}

/// Minimal always-on-top overlay (controls hint + production tallies).
fn draw_hud_overlay(factory: &game_logic::view::FactorySnapshot, hover: TilePos) {
    draw_text(
        "Tap: place / select   drag: pan   pinch: zoom   Mine btn: remove",
        12.0,
        20.0,
        18.0,
        WHITE,
    );
    draw_text(
        &format!("tile ({}, {})", hover.x, hover.y),
        12.0,
        40.0,
        16.0,
        GRAY,
    );
    let mut ty = 62.0;
    draw_text("Produced:", 12.0, ty, 18.0, WHITE);
    ty += 20.0;
    for (kind, count) in &factory.produced {
        draw_text(
            &format!("{}: {}", kind.name(), count),
            12.0,
            ty,
            16.0,
            WHITE,
        );
        ty += 18.0;
    }
}

struct InspectorInfo {
    kind: BuildingKind,
    title: String,
    lines: Vec<String>,
    assembler_recipe: Option<RecipeId>,
}

/// Gather a read-only description of a building for the inspector panel.
fn inspector_info(world: &World, id: InstanceId) -> Option<InspectorInfo> {
    let inst = world.tile_grid.instances.get(&id)?;
    let kind = BuildingKind::from_spec_id(inst.spec_id)?;
    let state = world.factory.state(id)?;
    let mut lines = Vec::new();
    let mut assembler_recipe = None;
    match state {
        BuildingState::Belt { item } => lines.push(match item {
            Some(it) => format!("carrying {}", it.kind.name()),
            None => "empty".to_string(),
        }),
        BuildingState::Miner { fuel, output, .. } => {
            lines.push(format!("fuel: {fuel} coal"));
            lines.push(match output {
                Some(o) => format!("holding {}", o.name()),
                None => "mining...".to_string(),
            });
        }
        BuildingState::Furnace {
            fuel,
            input,
            output,
            ..
        } => {
            lines.push(format!("fuel: {fuel} coal"));
            lines.push(match input {
                Some((k, n)) => format!("in: {} x{}", k.name(), n),
                None => "in: empty".to_string(),
            });
            lines.push(match output {
                Some((k, n)) => format!("out: {} x{}", k.name(), n),
                None => "out: empty".to_string(),
            });
        }
        BuildingState::Inserter { holding, .. } => lines.push(match holding {
            Some(k) => format!("holding {}", k.name()),
            None => "reaching...".to_string(),
        }),
        BuildingState::Assembler {
            recipe: sel,
            inputs,
            outputs,
            ..
        } => {
            assembler_recipe = *sel;
            match sel.and_then(recipe) {
                Some(r) => {
                    lines.push(format!("recipe: {}", r.name));
                    lines.push(format!("needs: {}", ingredients_label(r)));
                }
                None => lines.push("recipe: none".to_string()),
            }
            let have: u32 = inputs.values().sum();
            let out: u32 = outputs.values().sum();
            lines.push(format!("buffered in:{have} out:{out}"));
        }
        BuildingState::Chest { items } => {
            if items.is_empty() {
                lines.push("empty".to_string());
            } else {
                for (k, n) in items {
                    lines.push(format!("{} x{}", k.name(), n));
                }
            }
        }
    }
    Some(InspectorInfo {
        kind,
        title: kind.name().to_string(),
        lines,
        assembler_recipe,
    })
}

fn short(name: &str) -> String {
    name.chars().take(5).collect()
}
