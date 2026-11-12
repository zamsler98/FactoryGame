use super::RenderEntity;
use game_logic::EntityInfo;
use macroquad::{
    color::WHITE,
    math::vec2,
    texture::{DrawTextureParams, FilterMode, Texture2D, draw_texture_ex, load_texture},
};

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
    fn render(&self, entity: &EntityInfo) {
        let rect_w = 100.0;
        let rect_h = 100.0;
        let position = entity.position.as_vec2f();
        draw_texture_ex(
            &self.texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect_w, rect_h)),
                ..Default::default()
            },
        );
    }
}
