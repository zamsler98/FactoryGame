use game_core::Entity;
use game_logic::EntityRegistry;

pub mod entity_renderer;
pub mod miner_renderer;

pub use entity_renderer::EntityRenderer;

pub trait RenderEntity {
    fn render(&self, entity: Entity, registry: &EntityRegistry);
}
