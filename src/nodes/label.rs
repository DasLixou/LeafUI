use taffy::prelude::Layout;

use crate::{Blossom, Leaf, Shrub};

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

impl Blossom for Label {
    fn render(&self, _shrub: &Shrub, layout: &Layout, _children: &[Leaf]) {
        println!("Render Label: {layout:?}");
    }
}
