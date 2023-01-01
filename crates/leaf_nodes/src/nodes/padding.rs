use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct Padding {
    children: Vec<LeafID>,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Padding {
    pub const fn padding(mut self, left: i32, right: i32, top: i32, bottom: i32) -> Self {
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
        self
    }

    pub fn add_child(mut self, child: LeafID) -> Self {
        self.children.push(child);
        self
    }
}

impl Leaf for Padding {
    fn new() -> Self
    where
        Self: Sized,
    {
        Padding {
            children: vec![],
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }

    fn layout(&self, _shrub: &mut Shrub) -> LeafID {
        LeafID::UNKNOWN
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
