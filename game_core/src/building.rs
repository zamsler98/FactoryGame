//! Building catalog: every placeable building kind and its static spec.
//!
//! To add a new building, add a variant, extend `ALL` and the match arms
//! here, then give it runtime behavior in `factory.rs` (a `BuildingState`
//! variant plus tick arms).

use crate::{BuildingSpec, Size2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildingKind {
    Conveyor,
    Miner,
    Smelter,
}

impl BuildingKind {
    pub const ALL: [BuildingKind; 3] = [
        BuildingKind::Conveyor,
        BuildingKind::Miner,
        BuildingKind::Smelter,
    ];

    /// Stable numeric id stored in `BuildingInstance::spec_id`.
    pub fn spec_id(self) -> u32 {
        match self {
            BuildingKind::Conveyor => 1,
            BuildingKind::Miner => 2,
            BuildingKind::Smelter => 3,
        }
    }

    pub fn from_spec_id(id: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|k| k.spec_id() == id)
    }

    pub fn name(self) -> &'static str {
        match self {
            BuildingKind::Conveyor => "Conveyor",
            BuildingKind::Miner => "Miner",
            BuildingKind::Smelter => "Smelter",
        }
    }

    pub fn spec(self) -> BuildingSpec {
        BuildingSpec {
            spec_id: self.spec_id(),
            size: Size2 { w: 1, h: 1 },
        }
    }
}
