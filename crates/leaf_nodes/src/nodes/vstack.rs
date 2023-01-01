use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct VStack {
    children: Vec<LeafID>,
    id: LeafID,
}

impl VStack {
    pub fn add_child(mut self, child: LeafID) -> Self {
        self.children.push(child);
        self
    }
}

impl Leaf for VStack {
    fn new(cx: &mut Shrub) -> Self
    where
        Self: Sized,
    {
        VStack {
            children: vec![],
            id: cx.register_leaf(),
        }
    }

    fn layout(&self, cx: &mut Shrub) -> LeafID {
        LeafID::Unknown
    }

    fn id(&self) -> LeafID {
        self.id
    }
}
