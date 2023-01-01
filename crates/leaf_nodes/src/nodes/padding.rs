use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct Padding {
    children: Vec<LeafID>,
    id: LeafID,
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
    fn new(cx: &mut Shrub) -> Self
    where
        Self: Sized,
    {
        Padding {
            children: vec![],
            id: cx.register_leaf(),
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }

    fn layout(&self, cx: &mut Shrub) -> LeafID {
        LeafID::UNKNOWN
    }

    fn id(&self) -> LeafID {
        self.id
    }
}
