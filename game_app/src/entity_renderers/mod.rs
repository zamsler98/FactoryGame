use game_core::Entity;

pub mod miner_renderer;

pub trait RenderEntity {
    fn render(&self, entity: &Entity);
}
