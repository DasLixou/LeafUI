use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct Label {
    text: String,
}

impl Label {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl Leaf for Label {
    fn new() -> Self
    where
        Self: Sized,
    {
        Label {
            text: String::new(),
        }
    }

    fn layout(&self, _shrub: &mut Shrub) -> LeafID {
        LeafID::UNKNOWN
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
