use crate::{BuildingType, GridCoords};

#[derive(Debug)]
pub struct Building {
    pub position: GridCoords,
    pub building_type: BuildingType,
}
