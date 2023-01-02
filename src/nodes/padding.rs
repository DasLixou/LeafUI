use taffy::{
    prelude::{Node, Rect},
    style::{Dimension, Style},
    Taffy,
};

use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct Padding {
    child: LeafID,
    padding: Rect<Dimension>,
}

impl Padding {
    pub const fn new(padding: Rect<Dimension>) -> Self {
        Padding {
            child: LeafID::UNKNOWN,
            padding,
        }
    }

    pub const fn padding(mut self, padding: Rect<Dimension>) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_child(mut self, child: LeafID) -> Self {
        self.child = child;
        self
    }
}

impl Leaf for Padding {
    fn layout(&self, taffy: &mut Taffy) -> Node {
        taffy
            .new_leaf(Style {
                padding: self.padding,
                ..Default::default()
            })
            .unwrap()
    }

    fn design(&self, _shrub: &mut Shrub) -> LeafID {
        LeafID::UNKNOWN
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
