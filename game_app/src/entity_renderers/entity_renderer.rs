use super::miner_renderer::MinerRenderer;
use super::RenderEntity;
use game_core::{Entity, EntityType};

/// Owns one renderer per entity type and dispatches each entity to the right one.
/// Renderers are shared across every entity of their type so a texture is loaded once.
pub struct EntityRenderer {
    miner: MinerRenderer,
}

impl EntityRenderer {
    pub async fn load() -> Self {
        Self {
            miner: MinerRenderer::load().await,
        }
    }

    pub fn render(&self, entity: &Entity) {
        match entity.entity_type {
            EntityType::Miner => self.miner.render(entity),
        }
    }
}
