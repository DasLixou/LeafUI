use taffy::{prelude::Node, style::Style, Taffy};

use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label { text: text.into() }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl Leaf for Label {
    fn layout(&self, taffy: &mut Taffy) -> Node {
        taffy
            .new_leaf(Style {
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
