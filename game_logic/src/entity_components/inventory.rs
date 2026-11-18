#[derive(Debug)]
pub struct Inventory {
    num_items: u32,
    capacity: u32,
}

impl Inventory {
    pub fn new(capacity: u32) -> Self {
        Self {
            num_items: 0,
            capacity,
        }
    }

    pub fn num_items(&self) -> u32 {
        self.num_items
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn is_full(&self) -> bool {
        self.num_items >= self.capacity
    }

    pub fn add(&mut self) -> bool {
        if self.is_full() {
            return false;
        }
        self.num_items += 1;
        true
    }
}
