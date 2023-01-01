use hashbrown::HashMap;

use crate::Leaf;

#[derive(Debug, Clone, Copy)]
pub struct LeafID(u64);

impl LeafID {
    pub const UNKNOWN: LeafID = LeafID(0);
}

pub struct Shrub {
    counter: u64,
    leaves: HashMap<LeafID, Box<dyn Leaf>>,
}

impl Shrub {
    pub fn new() -> Self {
        Self {
            counter: 0,
            leaves: HashMap::new(),
        }
    }

    pub fn register_leaf(&mut self) -> LeafID {
        self.counter += 1;
        LeafID(self.counter)
    }
}
