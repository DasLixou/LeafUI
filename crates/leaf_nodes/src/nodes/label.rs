use crate::Leaf;

#[derive(Debug)]
pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label { text: text.into() }
    }
}

impl Leaf for Label {
    fn layout(&mut self) -> Option<Box<dyn Leaf>> {
        None
    }
}
