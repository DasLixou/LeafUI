use taffy::{
    prelude::{Node, Rect},
    style::{Dimension, Style},
    Taffy,
};

use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct Padding {
    child: Option<LeafID>,
    padding: Rect<Dimension>,
}

impl Padding {
    pub const fn new(padding: Rect<Dimension>) -> Self {
        Padding {
            child: None,
            padding,
        }
    }

    pub const fn padding(mut self, padding: Rect<Dimension>) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_child(mut self, child: LeafID) -> Self {
        self.child = Some(child);
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

    fn design(&self, _shrub: &mut Shrub) -> Option<LeafID> {
        None
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
