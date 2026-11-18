pub const TICKS_TO_MINE: u64 = 240;
pub const MINER_INVENTORY_CAPACITY: u32 = 5;

#[derive(Debug)]
pub struct Miner {
    next_mine_tick: u64,
}

impl Miner {
    pub fn new(current_tick: u64) -> Self {
        Self {
            next_mine_tick: current_tick + TICKS_TO_MINE,
        }
    }

    pub fn try_mine(&mut self, tick: u64) -> bool {
        if tick < self.next_mine_tick {
            return false;
        }
        self.next_mine_tick = tick + TICKS_TO_MINE;
        true
    }
}
