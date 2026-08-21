use macroquad::texture::{load_texture, FilterMode, Texture2D};

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

// impl RenderEntity for MinerRenderer {
//     fn render(&self, entity: &Entity) {
//        draw_texture_ex(self.texture, entity.
//     }
// }
