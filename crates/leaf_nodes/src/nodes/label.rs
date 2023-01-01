use crate::{Leaf, LeafID, Shrub};

#[derive(Debug)]
pub struct Label {
    text: String,
    id: LeafID,
}

impl Label {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl Leaf for Label {
    fn new(cx: &mut Shrub) -> Self
    where
        Self: Sized,
    {
        Label {
            text: String::new(),
            id: cx.register_leaf(),
        }
    }

    fn layout(&self, cx: &mut Shrub) -> LeafID {
        LeafID::UNKNOWN
    }

    fn id(&self) -> LeafID {
        self.id
    }
}
