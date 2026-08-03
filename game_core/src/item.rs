//! Item definitions: everything that can sit in a machine or move on a belt —
//! raw resources, smelted plates, and intermediate products. Buildings you can
//! place are items too — see `building.rs` for the `ItemKind` <-> `BuildingKind`
//! map (buildings place freely, so this map is only used for names/colors).
//!
//! To add a new item, add a variant, give it a `name`, add it to `ALL`, and
//! (if machines craft it) add a recipe in `recipe.rs`.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemKind {
    // Raw resources (mined from the ground)
    IronOre,
    CopperOre,
    Coal,
    Stone,
    // Smelted in a furnace
    IronPlate,
    CopperPlate,
    StoneBrick,
    // Intermediate products (assembled)
    IronGearWheel,
    CopperCable,
    ElectronicCircuit,
    // Placeable building items
    TransportBelt,
    BurnerMiningDrill,
    StoneFurnace,
    Inserter,
    AssemblingMachine,
    WoodenChest,
}

impl ItemKind {
    /// Every item, in a stable display order (raw -> processed -> buildings).
    pub const ALL: [ItemKind; 16] = [
        ItemKind::IronOre,
        ItemKind::CopperOre,
        ItemKind::Coal,
        ItemKind::Stone,
        ItemKind::IronPlate,
        ItemKind::CopperPlate,
        ItemKind::StoneBrick,
        ItemKind::IronGearWheel,
        ItemKind::CopperCable,
        ItemKind::ElectronicCircuit,
        ItemKind::TransportBelt,
        ItemKind::BurnerMiningDrill,
        ItemKind::StoneFurnace,
        ItemKind::Inserter,
        ItemKind::AssemblingMachine,
        ItemKind::WoodenChest,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ItemKind::IronOre => "Iron Ore",
            ItemKind::CopperOre => "Copper Ore",
            ItemKind::Coal => "Coal",
            ItemKind::Stone => "Stone",
            ItemKind::IronPlate => "Iron Plate",
            ItemKind::CopperPlate => "Copper Plate",
            ItemKind::StoneBrick => "Stone Brick",
            ItemKind::IronGearWheel => "Iron Gear Wheel",
            ItemKind::CopperCable => "Copper Cable",
            ItemKind::ElectronicCircuit => "Electronic Circuit",
            ItemKind::TransportBelt => "Transport Belt",
            ItemKind::BurnerMiningDrill => "Burner Mining Drill",
            ItemKind::StoneFurnace => "Stone Furnace",
            ItemKind::Inserter => "Inserter",
            ItemKind::AssemblingMachine => "Assembling Machine",
            ItemKind::WoodenChest => "Wooden Chest",
        }
    }
}
