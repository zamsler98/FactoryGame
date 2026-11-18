use super::RenderEntity;
use super::miner_renderer::MinerRenderer;
use game_core::Entity;
use game_logic::{BuildingType, EntityRegistry};

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

    pub fn render(&self, entity: Entity, registry: &EntityRegistry) {
        let Some(building) = registry.buildings.get(entity) else {
            return;
        };
        match building.building_type {
            BuildingType::Miner => self.miner.render(entity, registry),
        }
    }
}
