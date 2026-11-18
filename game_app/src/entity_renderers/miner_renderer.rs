use super::RenderEntity;
use game_core::Entity;
use game_logic::{EntityRegistry, TILE_SIZE};
use macroquad::{
    color::{BLACK, WHITE},
    math::vec2,
    shapes::{draw_circle, draw_circle_lines},
    texture::{DrawTextureParams, FilterMode, Texture2D, draw_texture_ex, load_texture},
};

const DOT_RADIUS: f32 = 4.0;
const DOT_SPACING: f32 = 12.0; // center-to-center
const DOT_BOTTOM_MARGIN: f32 = 8.0; // from miner bottom edge to dot centers
const DOT_OUTLINE_THICKNESS: f32 = 1.0;

pub struct MinerRenderer {
    pub texture: Texture2D,
}

impl MinerRenderer {
    pub async fn load() -> Self {
        let texture = load_texture("assets/miner_sprite.png")
            .await
            .expect("failed to load assets/miner_sprite.png");
        texture.set_filter(FilterMode::Nearest);
        Self { texture }
    }
}

impl RenderEntity for MinerRenderer {
    fn render(&self, entity: Entity, registry: &EntityRegistry) {
        let Some(building) = registry.buildings.get(entity) else {
            return;
        };
        let size = building.building_type.size();
        let rect_w = (TILE_SIZE as f32) * size.x as f32;
        let rect_h = (TILE_SIZE as f32) * size.y as f32;
        let position = building.position.world_origin();
        draw_texture_ex(
            &self.texture,
            position.x as f32,
            position.y as f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect_w, rect_h)),
                ..Default::default()
            },
        );

        if let Some(inventory) = registry.inventories.get(entity) {
            let capacity = inventory.capacity();
            if capacity == 0 {
                return;
            }
            let y = position.y as f32 + rect_h - DOT_BOTTOM_MARGIN;
            let first_x =
                position.x as f32 + rect_w / 2.0 - (capacity - 1) as f32 * DOT_SPACING / 2.0;
            for i in 0..capacity {
                let x = first_x + i as f32 * DOT_SPACING;
                if i < inventory.num_items() {
                    draw_circle(x, y, DOT_RADIUS, BLACK);
                } else {
                    draw_circle_lines(x, y, DOT_RADIUS, DOT_OUTLINE_THICKNESS, BLACK);
                }
            }
        }
    }
}
