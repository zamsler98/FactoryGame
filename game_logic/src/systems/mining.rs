use crate::EntityRegistry;

pub(crate) fn mining_system(registry: &mut EntityRegistry, tick: u64) {
    for (entity, miner) in registry.miners.iter_mut() {
        let Some(inventory) = registry.inventories.get_mut(entity) else {
            continue;
        };
        if inventory.is_full() {
            continue;
        }
        if miner.try_mine(tick) {
            inventory.add();
        }
    }
}
