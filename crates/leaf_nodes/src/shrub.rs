#[derive(Debug, Clone, Copy)]
pub struct LeafID(u64);

impl LeafID {
    pub const Unknown: LeafID = LeafID(0);
}

pub struct Shrub;

impl Shrub {
    pub fn new() -> Self {
        Self
    }

    pub fn register_leaf(&mut self) -> LeafID {
        LeafID(1)
    }
}
