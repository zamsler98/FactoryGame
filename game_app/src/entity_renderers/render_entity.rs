use game_logic::Entity;

trait RenderEntity {
    fn render(&self, entity: &Entity);
}
