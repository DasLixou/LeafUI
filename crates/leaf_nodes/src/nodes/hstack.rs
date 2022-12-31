use crate::{Branch, Leaf, Style};

pub struct HStack(Style, Box<dyn Branch>);

impl HStack {
    pub fn new() -> Self {
        HStack(Style::default(), Box::new(()))
    }

    pub fn style(mut self, style: Style) -> Self {
        self.0 = style;
        self
    }

    pub fn children(mut self, branch: Box<dyn Branch>) -> Self {
        self.1 = branch;
        self
    }
}

impl Leaf for HStack {}
