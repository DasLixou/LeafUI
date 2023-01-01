use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct Padding {
    child: LeafID,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Padding {
    pub const fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        Padding {
            child: LeafID::UNKNOWN,
            left,
            right,
            top,
            bottom,
        }
    }

    pub const fn padding(mut self, left: i32, right: i32, top: i32, bottom: i32) -> Self {
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
        self
    }

    pub fn set_child(mut self, child: LeafID) -> Self {
        self.child = child;
        self
    }
}

impl Leaf for Padding {
    fn layout(&self, _shrub: &mut Shrub) -> LeafID {
        LeafID::UNKNOWN
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
