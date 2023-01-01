use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct HStack {
    children: Vec<LeafID>,
}

impl HStack {
    pub fn add_child(mut self, child: LeafID) -> Self {
        self.children.push(child);
        self
    }
}

impl Leaf for HStack {
    fn new() -> Self
    where
        Self: Sized,
    {
        HStack { children: vec![] }
    }

    fn layout(&self, _shrub: &mut Shrub) -> LeafID {
        LeafID::UNKNOWN
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
