//! Factory simulation: per-building runtime state and the item-flow tick.
//!
//! Every placed `BuildingInstance` gets a `BuildingState` keyed by its
//! `InstanceId`. `Factory::tick` runs two deterministic phases over the
//! instances in id order:
//!
//! 1. progress — miners burn fuel and mine the patch under them, belts slide
//!    their item, furnaces/assemblers burn fuel/craft, inserters swing.
//! 2. transfer — items move between buildings. Miners drop directly onto the
//!    tile they face; belts hand off to the next belt; everything else is
//!    moved by inserters (grab from behind, drop in front) — just like
//!    Factorio, where belts can't load machines on their own.
//!
//! To give a new `BuildingKind` behavior, add a `BuildingState` variant, an
//! arm in `BuildingState::initial`, arms in `advance_progress`, and arms in
//! the item-flow helpers (`offer`, `take_one`, `can_accept`, `accept`).

use std::collections::HashMap;

use crate::{
    recipe, smelting_recipe_for, BuildingInstance, BuildingKind, InstanceId, ItemKind, RecipeId,
    ResourceLayer, TileGrid, TilePos,
};

/// Seconds for a miner to produce one ore.
pub const MINER_PERIOD: f32 = 1.0;
/// Belt speed in tiles per second.
pub const CONVEYOR_SPEED: f32 = 2.0;
/// Seconds for an inserter to move one item.
pub const INSERTER_SWING: f32 = 0.8;
/// Max fuel units a burner building holds.
pub const FUEL_CAP: u32 = 5;
/// Max items a furnace holds in its input (and output) buffer.
pub const FURNACE_INPUT_CAP: u32 = 5;
pub const FURNACE_OUTPUT_CAP: u32 = 5;
/// Max of any single item an assembler buffers on each side.
pub const ASSEMBLER_INPUT_CAP: u32 = 50;
pub const ASSEMBLER_OUTPUT_CAP: u32 = 50;
/// Max of any single item a chest holds.
pub const CHEST_CAP: u32 = 1000;

#[derive(Clone, Copy, Debug)]
pub struct ConveyorItem {
    pub kind: ItemKind,
    /// 0.0 = just entered the tile, 1.0 = waiting at the exit edge.
    pub progress: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct CraftInProgress {
    pub recipe: RecipeId,
    /// 0.0..1.0
    pub progress: f32,
}

#[derive(Clone, Debug)]
pub enum BuildingState {
    Belt {
        item: Option<ConveyorItem>,
    },
    Miner {
        /// Coal units waiting in the fuel slot.
        fuel: u32,
        /// Energy (seconds) left in the currently burning fuel.
        burn: f32,
        /// Mining progress 0.0..1.0; paused while `output` is occupied.
        progress: f32,
        output: Option<ItemKind>,
    },
    Furnace {
        fuel: u32,
        burn: f32,
        /// Input buffer: one stack of a single ore kind.
        input: Option<(ItemKind, u32)>,
        craft: Option<CraftInProgress>,
        /// Output buffer: one stack of a single item kind.
        output: Option<(ItemKind, u32)>,
    },
    Inserter {
        /// The item currently held mid-swing, if any.
        holding: Option<ItemKind>,
        /// Swing progress 0.0..1.0 while holding.
        progress: f32,
    },
    Assembler {
        recipe: Option<RecipeId>,
        inputs: HashMap<ItemKind, u32>,
        craft: Option<CraftInProgress>,
        outputs: HashMap<ItemKind, u32>,
    },
    Chest {
        items: HashMap<ItemKind, u32>,
    },
}

impl BuildingState {
    pub fn initial(kind: BuildingKind) -> Self {
        match kind {
            BuildingKind::Belt => BuildingState::Belt { item: None },
            BuildingKind::Miner => BuildingState::Miner {
                fuel: 0,
                burn: 0.0,
                progress: 0.0,
                output: None,
            },
            BuildingKind::Furnace => BuildingState::Furnace {
                fuel: 0,
                burn: 0.0,
                input: None,
                craft: None,
                output: None,
            },
            BuildingKind::Inserter => BuildingState::Inserter {
                holding: None,
                progress: 0.0,
            },
            BuildingKind::Assembler => BuildingState::Assembler {
                recipe: None,
                inputs: HashMap::new(),
                craft: None,
                outputs: HashMap::new(),
            },
            BuildingKind::Chest => BuildingState::Chest {
                items: HashMap::new(),
            },
        }
    }
}

/// Runtime state of every building plus global production tallies.
#[derive(Default)]
pub struct Factory {
    states: HashMap<InstanceId, BuildingState>,
    /// Total items completed by crafting machines, by kind.
    pub produced: HashMap<ItemKind, u64>,
}

impl Factory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self, id: InstanceId) -> Option<&BuildingState> {
        self.states.get(&id)
    }

    pub fn state_mut(&mut self, id: InstanceId) -> Option<&mut BuildingState> {
        self.states.get_mut(&id)
    }

    pub fn states(&self) -> impl Iterator<Item = (InstanceId, &BuildingState)> {
        self.states.iter().map(|(id, s)| (*id, s))
    }

    pub fn on_building_placed(&mut self, inst: &BuildingInstance) {
        if let Some(kind) = BuildingKind::from_spec_id(inst.spec_id) {
            self.states.insert(inst.id, BuildingState::initial(kind));
        }
    }

    pub fn on_building_removed(&mut self, id: InstanceId) -> Option<BuildingState> {
        self.states.remove(&id)
    }

    /// Advance the simulation by `dt` seconds.
    pub fn tick(&mut self, grid: &TileGrid, resources: &mut ResourceLayer, dt: f32) {
        let mut ids: Vec<InstanceId> = self.states.keys().copied().collect();
        ids.sort_unstable();

        self.advance_progress(grid, resources, &ids, dt);
        self.run_transfers(grid, &ids);
    }

    fn advance_progress(
        &mut self,
        grid: &TileGrid,
        resources: &mut ResourceLayer,
        ids: &[InstanceId],
        dt: f32,
    ) {
        for &id in ids {
            let origin = grid.instances.get(&id).map(|i| i.origin);
            let Some(state) = self.states.get_mut(&id) else {
                continue;
            };
            match state {
                BuildingState::Belt { item } => {
                    if let Some(item) = item {
                        item.progress = (item.progress + CONVEYOR_SPEED * dt).min(1.0);
                    }
                }
                BuildingState::Miner {
                    fuel,
                    burn,
                    progress,
                    output,
                } => {
                    if output.is_none() {
                        refuel(fuel, burn);
                        if *burn > 0.0 {
                            *progress += dt / MINER_PERIOD;
                            *burn -= dt;
                            if *progress >= 1.0 {
                                *progress = 0.0;
                                if let Some(pos) = origin {
                                    *output = resources.mine(pos);
                                }
                            }
                        }
                    }
                }
                BuildingState::Furnace {
                    fuel,
                    burn,
                    input,
                    craft,
                    output,
                } => {
                    // Start a craft if idle and the input ore has a recipe.
                    if craft.is_none() {
                        if let Some((item, count)) = input {
                            if let Some(r) = smelting_recipe_for(*item) {
                                let need = r.inputs[0].count;
                                let out_item = r.primary_output();
                                let out_ok = match output {
                                    None => true,
                                    Some((k, n)) => *k == out_item && *n < FURNACE_OUTPUT_CAP,
                                };
                                if *count >= need && out_ok {
                                    *count -= need;
                                    if *count == 0 {
                                        *input = None;
                                    }
                                    *craft = Some(CraftInProgress {
                                        recipe: r.id,
                                        progress: 0.0,
                                    });
                                }
                            }
                        }
                    }
                    // Burn fuel to advance the active craft.
                    if let Some(c) = craft {
                        refuel(fuel, burn);
                        if *burn > 0.0 {
                            let dur = recipe(c.recipe).map_or(1.0, |r| r.duration);
                            c.progress += dt / dur;
                            *burn -= dt;
                            if c.progress >= 1.0 {
                                if let Some(r) = recipe(c.recipe) {
                                    for out in r.outputs {
                                        deposit_stack(output, out.item, out.count);
                                        *self.produced.entry(out.item).or_insert(0) +=
                                            out.count as u64;
                                    }
                                }
                                *craft = None;
                            }
                        }
                    }
                }
                BuildingState::Inserter { holding, progress } => {
                    if holding.is_some() {
                        *progress = (*progress + dt / INSERTER_SWING).min(1.0);
                    }
                }
                BuildingState::Assembler {
                    recipe: sel,
                    inputs,
                    craft,
                    outputs,
                } => {
                    if let Some(rid) = *sel {
                        if let Some(r) = recipe(rid) {
                            if craft.is_none() {
                                let have = r
                                    .inputs
                                    .iter()
                                    .all(|i| inputs.get(&i.item).copied().unwrap_or(0) >= i.count);
                                let room = r.outputs.iter().all(|o| {
                                    outputs.get(&o.item).copied().unwrap_or(0)
                                        < ASSEMBLER_OUTPUT_CAP
                                });
                                if have && room {
                                    for i in r.inputs {
                                        let e = inputs.entry(i.item).or_insert(0);
                                        *e -= i.count;
                                        if *e == 0 {
                                            inputs.remove(&i.item);
                                        }
                                    }
                                    *craft = Some(CraftInProgress {
                                        recipe: rid,
                                        progress: 0.0,
                                    });
                                }
                            }
                            if let Some(c) = craft {
                                c.progress += dt / r.duration;
                                if c.progress >= 1.0 {
                                    for o in r.outputs {
                                        *outputs.entry(o.item).or_insert(0) += o.count;
                                        *self.produced.entry(o.item).or_insert(0) += o.count as u64;
                                    }
                                    *craft = None;
                                }
                            }
                        }
                    }
                }
                BuildingState::Chest { .. } => {}
            }
        }
    }

    fn run_transfers(&mut self, grid: &TileGrid, ids: &[InstanceId]) {
        for &id in ids {
            let Some(inst) = grid.instances.get(&id) else {
                continue;
            };
            let Some(kind) = BuildingKind::from_spec_id(inst.spec_id) else {
                continue;
            };
            let (dx, dy) = inst.rotation.dir();
            let front = TilePos {
                x: inst.origin.x + dx,
                y: inst.origin.y + dy,
            };
            match kind {
                BuildingKind::Miner => self.push_front(grid, id, front),
                BuildingKind::Belt => self.belt_handoff(grid, id, front),
                BuildingKind::Inserter => {
                    let back = TilePos {
                        x: inst.origin.x - dx,
                        y: inst.origin.y - dy,
                    };
                    self.inserter_step(grid, id, back, front);
                }
                // Furnaces, assemblers, and chests never push on their own —
                // an inserter must pull from them.
                BuildingKind::Furnace | BuildingKind::Assembler | BuildingKind::Chest => {}
            }
        }
    }

    /// Miner drops its output directly onto whatever building it faces.
    fn push_front(&mut self, grid: &TileGrid, id: InstanceId, front: TilePos) {
        let Some(item) = self.states.get(&id).and_then(offer) else {
            return;
        };
        let Some(tid) = grid.tile_occupant(front) else {
            return;
        };
        if tid == id {
            return;
        }
        if self.states.get_mut(&tid).is_some_and(|t| accept(t, item)) {
            if let Some(s) = self.states.get_mut(&id) {
                take_one(s, item);
            }
        }
    }

    /// Belt hands its item to the next belt in front once it reaches the exit.
    fn belt_handoff(&mut self, grid: &TileGrid, id: InstanceId, front: TilePos) {
        let item = match self.states.get(&id) {
            Some(BuildingState::Belt { item: Some(it) }) if it.progress >= 1.0 => it.kind,
            _ => return,
        };
        let Some(tid) = grid.tile_occupant(front) else {
            return;
        };
        if tid == id {
            return;
        }
        // Belts only feed other belts directly.
        let target_is_belt = grid
            .instances
            .get(&tid)
            .and_then(|i| BuildingKind::from_spec_id(i.spec_id))
            .is_some_and(|k| k == BuildingKind::Belt);
        if !target_is_belt {
            return;
        }
        if self.states.get_mut(&tid).is_some_and(|t| accept(t, item)) {
            if let Some(s) = self.states.get_mut(&id) {
                take_one(s, item);
            }
        }
    }

    /// Inserter: grab from the tile behind, drop on the tile in front.
    fn inserter_step(&mut self, grid: &TileGrid, id: InstanceId, back: TilePos, front: TilePos) {
        let (holding, ready) = match self.states.get(&id) {
            Some(BuildingState::Inserter { holding, progress }) => (*holding, *progress >= 1.0),
            _ => return,
        };
        match holding {
            None => {
                // Pick up only if there is something behind and the target in
                // front could take it (so we don't strand the item).
                let Some(sid) = grid.tile_occupant(back).filter(|&s| s != id) else {
                    return;
                };
                let Some(item) = self.states.get(&sid).and_then(offer) else {
                    return;
                };
                let can_drop = grid
                    .tile_occupant(front)
                    .filter(|&t| t != id)
                    .and_then(|tid| self.states.get(&tid))
                    .is_some_and(|t| can_accept(t, item));
                if !can_drop {
                    return;
                }
                if let Some(s) = self.states.get_mut(&sid) {
                    take_one(s, item);
                }
                if let Some(BuildingState::Inserter { holding, progress }) =
                    self.states.get_mut(&id)
                {
                    *holding = Some(item);
                    *progress = 0.0;
                }
            }
            Some(item) if ready => {
                let Some(tid) = grid.tile_occupant(front).filter(|&t| t != id) else {
                    return;
                };
                if self.states.get_mut(&tid).is_some_and(|t| accept(t, item)) {
                    if let Some(BuildingState::Inserter { holding, progress }) =
                        self.states.get_mut(&id)
                    {
                        *holding = None;
                        *progress = 0.0;
                    }
                }
            }
            Some(_) => {}
        }
    }
}

/// Refuel a burner: if nothing is burning and coal is queued, start a unit.
fn refuel(fuel: &mut u32, burn: &mut f32) {
    if *burn <= 0.0 && *fuel > 0 {
        *fuel -= 1;
        *burn += ItemKind::Coal.fuel_energy().unwrap_or(0.0);
    }
}

/// Add `count` of `item` to a single-stack buffer that already holds `item`
/// (or is empty).
fn deposit_stack(slot: &mut Option<(ItemKind, u32)>, item: ItemKind, count: u32) {
    match slot {
        None => *slot = Some((item, count)),
        Some((k, n)) if *k == item => *n += count,
        // Never reached: callers check the stack matches before crafting.
        Some(_) => {}
    }
}

/// The item a building currently offers to be taken from its output.
fn offer(state: &BuildingState) -> Option<ItemKind> {
    match state {
        BuildingState::Belt { item } => item.map(|i| i.kind),
        BuildingState::Miner { output, .. } => *output,
        BuildingState::Furnace { output, .. } => output.map(|(k, _)| k),
        BuildingState::Assembler { outputs, .. } => first_item(outputs),
        BuildingState::Chest { items } => first_item(items),
        BuildingState::Inserter { .. } => None,
    }
}

/// Remove one `item` from a building's output after it has been taken.
fn take_one(state: &mut BuildingState, item: ItemKind) {
    match state {
        BuildingState::Belt { item: slot } => *slot = None,
        BuildingState::Miner { output, .. } => *output = None,
        BuildingState::Furnace { output, .. } => decrement_stack(output),
        BuildingState::Assembler { outputs, .. } => decrement_map(outputs, item),
        BuildingState::Chest { items } => decrement_map(items, item),
        BuildingState::Inserter { .. } => {}
    }
}

/// Whether a building can currently accept one `item` into its input.
fn can_accept(state: &BuildingState, item: ItemKind) -> bool {
    match state {
        BuildingState::Belt { item: slot } => slot.is_none(),
        BuildingState::Miner { fuel, .. } => item.is_fuel() && *fuel < FUEL_CAP,
        BuildingState::Furnace { fuel, input, .. } => furnace_accepts(item, *fuel, input),
        BuildingState::Assembler {
            recipe: sel,
            inputs,
            ..
        } => assembler_accepts(item, *sel, inputs),
        BuildingState::Chest { items } => items.get(&item).copied().unwrap_or(0) < CHEST_CAP,
        BuildingState::Inserter { .. } => false,
    }
}

/// Perform the acceptance of one `item`. Returns whether it happened.
fn accept(state: &mut BuildingState, item: ItemKind) -> bool {
    match state {
        BuildingState::Belt { item: slot } => {
            if slot.is_some() {
                return false;
            }
            *slot = Some(ConveyorItem {
                kind: item,
                progress: 0.0,
            });
            true
        }
        BuildingState::Miner { fuel, .. } => {
            if item.is_fuel() && *fuel < FUEL_CAP {
                *fuel += 1;
                true
            } else {
                false
            }
        }
        BuildingState::Furnace { fuel, input, .. } => {
            if !furnace_accepts(item, *fuel, input) {
                return false;
            }
            if item.is_fuel() && smelting_recipe_for(item).is_none() {
                *fuel += 1;
            } else {
                match input {
                    None => *input = Some((item, 1)),
                    Some((_, n)) => *n += 1,
                }
            }
            true
        }
        BuildingState::Assembler {
            recipe: sel,
            inputs,
            ..
        } => {
            if !assembler_accepts(item, *sel, inputs) {
                return false;
            }
            *inputs.entry(item).or_insert(0) += 1;
            true
        }
        BuildingState::Chest { items } => {
            let e = items.entry(item).or_insert(0);
            if *e >= CHEST_CAP {
                return false;
            }
            *e += 1;
            true
        }
        BuildingState::Inserter { .. } => false,
    }
}

fn furnace_accepts(item: ItemKind, fuel: u32, input: &Option<(ItemKind, u32)>) -> bool {
    // Fuel items with no smelting recipe (coal) go to the fuel slot.
    if item.is_fuel() && smelting_recipe_for(item).is_none() {
        return fuel < FUEL_CAP;
    }
    if smelting_recipe_for(item).is_none() {
        return false;
    }
    match input {
        None => true,
        Some((k, n)) => *k == item && *n < FURNACE_INPUT_CAP,
    }
}

fn assembler_accepts(
    item: ItemKind,
    sel: Option<RecipeId>,
    inputs: &HashMap<ItemKind, u32>,
) -> bool {
    let Some(r) = sel.and_then(recipe) else {
        return false;
    };
    if !r.inputs.iter().any(|i| i.item == item) {
        return false;
    }
    inputs.get(&item).copied().unwrap_or(0) < ASSEMBLER_INPUT_CAP
}

/// First non-empty item in a map, in stable `ItemKind::ALL` order.
fn first_item(map: &HashMap<ItemKind, u32>) -> Option<ItemKind> {
    ItemKind::ALL
        .iter()
        .copied()
        .find(|k| map.get(k).copied().unwrap_or(0) > 0)
}

fn decrement_map(map: &mut HashMap<ItemKind, u32>, item: ItemKind) {
    if let Some(n) = map.get_mut(&item) {
        *n -= 1;
        if *n == 0 {
            map.remove(&item);
        }
    }
}

fn decrement_stack(slot: &mut Option<(ItemKind, u32)>) {
    if let Some((_, n)) = slot {
        *n -= 1;
        if *n == 0 {
            *slot = None;
        }
    }
}
