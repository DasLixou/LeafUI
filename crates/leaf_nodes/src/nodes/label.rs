use crate::{Leaf, Style};

pub struct Label(String, Style);

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label(text.into(), Style::default())
    }

    pub fn style(mut self, style: Style) -> Self {
        self.1 = style;
        self
    }
}

impl Leaf for Label {}
