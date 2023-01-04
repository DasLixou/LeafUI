use taffy::prelude::Layout;

use crate::Blossom;

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
    fn render(&self, layout: &Layout) {
        println!("Render Label: {layout:?}");
    }
}
