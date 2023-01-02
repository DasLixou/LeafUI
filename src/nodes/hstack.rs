use taffy::{
    prelude::Node,
    style::{FlexDirection, Style},
    Taffy,
};

use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct HStack {
    children: Vec<LeafID>,
}

impl HStack {
    pub const fn new() -> Self {
        HStack { children: vec![] }
    }

    pub fn add_child(mut self, child: LeafID) -> Self {
        self.children.push(child);
        self
    }
}

impl Leaf for HStack {
    fn layout(&self, taffy: &mut Taffy) -> Node {
        taffy
            .new_leaf(Style {
                flex_direction: FlexDirection::Row,
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
