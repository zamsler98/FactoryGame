//! Factory simulation: per-building runtime state and the item-flow tick.
//!
//! Every placed `BuildingInstance` gets a `BuildingState` keyed by its
//! `InstanceId`. `Factory::tick` runs two deterministic phases over the
//! instances in id order:
//!
//! 1. progress — miners advance mining timers, belt items slide along their
//!    tile, smelters start/advance crafts
//! 2. transfer — every building with an item ready tries to push it into the
//!    building on the tile it faces
//!
//! To give a new `BuildingKind` behavior, add a `BuildingState` variant, an
//! arm in `BuildingState::initial`, and arms in the two tick phases (plus
//! `try_accept`/`take_offered` if it holds items).

use std::collections::HashMap;

use crate::{
    smelter_recipe_for, BuildingInstance, BuildingKind, InstanceId, ItemKind, TileGrid, TilePos,
};

/// Seconds for a miner to produce one ore.
pub const MINER_PERIOD: f32 = 1.0;
/// Conveyor speed in tiles per second.
pub const CONVEYOR_SPEED: f32 = 2.0;
/// Max items a smelter holds in its input (and output) buffer.
pub const SMELTER_STACK_CAP: u32 = 5;

#[derive(Clone, Copy, Debug)]
pub struct ConveyorItem {
    pub kind: ItemKind,
    /// 0.0 = just entered the tile, 1.0 = waiting at the exit edge.
    pub progress: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct CraftInProgress {
    pub output: ItemKind,
    /// 0.0..1.0
    pub progress: f32,
    pub duration: f32,
}

#[derive(Clone, Debug)]
pub enum BuildingState {
    Miner {
        /// Mining progress 0.0..1.0; paused while `output` is occupied.
        progress: f32,
        output: Option<ItemKind>,
    },
    Conveyor {
        item: Option<ConveyorItem>,
    },
    Smelter {
        /// Input buffer: one stack of a single item kind.
        input: Option<(ItemKind, u32)>,
        craft: Option<CraftInProgress>,
        /// Output buffer: one stack of a single item kind.
        output: Option<(ItemKind, u32)>,
    },
}

impl BuildingState {
    pub fn initial(kind: BuildingKind) -> Self {
        match kind {
            BuildingKind::Conveyor => BuildingState::Conveyor { item: None },
            BuildingKind::Miner => BuildingState::Miner {
                progress: 0.0,
                output: None,
            },
            BuildingKind::Smelter => BuildingState::Smelter {
                input: None,
                craft: None,
                output: None,
            },
        }
    }
}

/// Runtime state of every building plus global production tallies.
#[derive(Default)]
pub struct Factory {
    states: HashMap<InstanceId, BuildingState>,
    /// Total items completed by machines, by kind.
    pub produced: HashMap<ItemKind, u64>,
}

impl Factory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self, id: InstanceId) -> Option<&BuildingState> {
        self.states.get(&id)
    }

    pub fn states(&self) -> impl Iterator<Item = (InstanceId, &BuildingState)> {
        self.states.iter().map(|(id, s)| (*id, s))
    }

    pub fn on_building_placed(&mut self, inst: &BuildingInstance) {
        if let Some(kind) = BuildingKind::from_spec_id(inst.spec_id) {
            self.states.insert(inst.id, BuildingState::initial(kind));
        }
    }

    pub fn on_building_removed(&mut self, id: InstanceId) {
        self.states.remove(&id);
    }

    /// Advance the simulation by `dt` seconds.
    pub fn tick(&mut self, grid: &TileGrid, dt: f32) {
        let mut ids: Vec<InstanceId> = self.states.keys().copied().collect();
        ids.sort_unstable();

        self.advance_progress(&ids, dt);
        self.run_transfers(&ids, grid);
    }

    fn advance_progress(&mut self, ids: &[InstanceId], dt: f32) {
        for id in ids {
            let Some(state) = self.states.get_mut(id) else {
                continue;
            };
            match state {
                BuildingState::Miner { progress, output } => {
                    if output.is_none() {
                        *progress += dt / MINER_PERIOD;
                        if *progress >= 1.0 {
                            *progress = 0.0;
                            *output = Some(ItemKind::IronOre);
                        }
                    }
                }
                BuildingState::Conveyor { item } => {
                    if let Some(item) = item {
                        item.progress = (item.progress + CONVEYOR_SPEED * dt).min(1.0);
                    }
                }
                BuildingState::Smelter {
                    input,
                    craft,
                    output,
                } => {
                    if craft.is_none() {
                        if let Some((kind, count)) = input {
                            if let Some(recipe) = smelter_recipe_for(*kind) {
                                let output_has_room = match output {
                                    None => true,
                                    Some((k, n)) => *k == recipe.output && *n < SMELTER_STACK_CAP,
                                };
                                if output_has_room {
                                    *count -= 1;
                                    if *count == 0 {
                                        *input = None;
                                    }
                                    *craft = Some(CraftInProgress {
                                        output: recipe.output,
                                        progress: 0.0,
                                        duration: recipe.duration,
                                    });
                                }
                            }
                        }
                    }
                    if let Some(c) = craft {
                        c.progress += dt / c.duration;
                        if c.progress >= 1.0 {
                            match output {
                                None => *output = Some((c.output, 1)),
                                Some((k, n)) if *k == c.output => *n += 1,
                                // Can't happen: craft only starts when the
                                // output stack matches and has room.
                                Some(_) => {}
                            }
                            *self.produced.entry(c.output).or_insert(0) += 1;
                            *craft = None;
                        }
                    }
                }
            }
        }
    }

    fn run_transfers(&mut self, ids: &[InstanceId], grid: &TileGrid) {
        for &id in ids {
            let Some(inst) = grid.instances.get(&id) else {
                continue;
            };
            let offered = match self.states.get(&id) {
                Some(BuildingState::Miner {
                    output: Some(kind), ..
                }) => Some(*kind),
                Some(BuildingState::Conveyor { item: Some(item) }) if item.progress >= 1.0 => {
                    Some(item.kind)
                }
                Some(BuildingState::Smelter {
                    output: Some((kind, _)),
                    ..
                }) => Some(*kind),
                _ => None,
            };
            let Some(kind) = offered else {
                continue;
            };

            // For 1x1 buildings the output tile is the neighbor of the origin
            // in the facing direction. Multi-tile buildings will need a
            // footprint-edge calculation here.
            let (dx, dy) = inst.rotation.dir();
            let target_pos = TilePos {
                x: inst.origin.x + dx,
                y: inst.origin.y + dy,
            };
            let Some(target_id) = grid.tile_occupant(target_pos) else {
                continue;
            };
            if target_id == id {
                continue;
            }

            if self.try_accept(target_id, kind) {
                self.take_offered(id);
            }
        }
    }

    /// Try to insert `kind` into building `id`. Returns true when accepted.
    fn try_accept(&mut self, id: InstanceId, kind: ItemKind) -> bool {
        match self.states.get_mut(&id) {
            Some(BuildingState::Conveyor { item: item @ None }) => {
                *item = Some(ConveyorItem {
                    kind,
                    progress: 0.0,
                });
                true
            }
            Some(BuildingState::Smelter { input, .. }) => {
                if smelter_recipe_for(kind).is_none() {
                    return false;
                }
                match input {
                    None => {
                        *input = Some((kind, 1));
                        true
                    }
                    Some((k, n)) if *k == kind && *n < SMELTER_STACK_CAP => {
                        *n += 1;
                        true
                    }
                    Some(_) => false,
                }
            }
            _ => false,
        }
    }

    /// Remove the item a building offered after a successful transfer.
    fn take_offered(&mut self, id: InstanceId) {
        match self.states.get_mut(&id) {
            Some(BuildingState::Miner { output, .. }) => *output = None,
            Some(BuildingState::Conveyor { item }) => *item = None,
            Some(BuildingState::Smelter { output, .. }) => {
                if let Some((_, n)) = output {
                    *n -= 1;
                    if *n == 0 {
                        *output = None;
                    }
                }
            }
            _ => {}
        }
    }
}
