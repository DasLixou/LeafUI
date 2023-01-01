use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct HStack {
    children: Vec<LeafID>,
    id: LeafID,
}

impl HStack {
    pub fn add_child(mut self, child: LeafID) -> Self {
        self.children.push(child);
        self
    }
}

impl Leaf for HStack {
    fn new(cx: &mut Shrub) -> Self
    where
        Self: Sized,
    {
        HStack {
            children: vec![],
            id: cx.register_leaf(),
        }
    }

    fn layout(&self, cx: &mut Shrub) -> Option<Box<dyn Leaf>> {
        None
    }

    fn id(&self) -> crate::LeafID {
        self.id
    }
}
