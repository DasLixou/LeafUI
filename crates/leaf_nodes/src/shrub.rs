#[derive(Debug, Clone, Copy)]
pub struct LeafID(u64);

pub struct Shrub;

impl Shrub {
    pub fn register_leaf(&mut self) -> LeafID {
        LeafID(0)
    }
}
