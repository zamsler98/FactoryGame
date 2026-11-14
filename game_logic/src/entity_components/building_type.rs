use game_core::Vec2i;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildingType {
    Miner,
}

impl BuildingType {
    pub fn size(self) -> Vec2i {
        match self {
            BuildingType::Miner => Vec2i::new(2, 2),
        }
    }
}
