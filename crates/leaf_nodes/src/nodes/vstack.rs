use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct VStack {
    children: Vec<LeafID>,
}

impl VStack {
    pub const fn new() -> Self {
        VStack { children: vec![] }
    }

    pub fn add_child(mut self, child: LeafID) -> Self {
        self.children.push(child);
        self
    }
}

impl Leaf for VStack {
    fn layout(&self, _shrub: &mut Shrub) -> LeafID {
        LeafID::UNKNOWN
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
