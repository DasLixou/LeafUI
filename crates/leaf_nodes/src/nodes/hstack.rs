use crate::{Branch, Leaf, RenderResult, Style};

#[derive(Debug)]
pub struct HStack(Style, Vec<Box<dyn Leaf>>);

impl HStack {
    pub fn new() -> Self {
        HStack(Style::default(), vec![])
    }

    pub fn style(mut self, style: Style) -> Self {
        self.0 = style;
        self
    }

    pub fn children(mut self, branch: impl Branch) -> Self {
        self.1 = branch.resolve();
        self
    }
}

impl Leaf for HStack {
    fn render(&mut self) -> RenderResult {
        RenderResult::ToDo
    }
}
