const TICKS_TO_MINE: u64 = 240;

#[derive(Debug)]
pub struct Miner {
    num_items: i32,
    next_mine_tick: u64,
}

impl Miner {
    pub fn new(current_tick: u64) -> Self {
        Self {
            num_items: 0,
            next_mine_tick: current_tick + TICKS_TO_MINE,
        }
    }

    pub fn update(&mut self, current_tick: u64) {
        if current_tick == self.next_mine_tick {
            self.num_items += 1;
            self.next_mine_tick += TICKS_TO_MINE;
            let items = &self.num_items;
            log::debug!("Miner has mined item. Now has {items}");
        }
    }
}
