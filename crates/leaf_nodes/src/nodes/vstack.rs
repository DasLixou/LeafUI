use crate::{Branch, Leaf, RenderResult, Style};

#[derive(Debug)]
pub struct VStack(Style, Vec<Box<dyn Leaf>>);

impl VStack {
    pub fn new() -> Self {
        VStack(Style::default(), vec![])
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

impl Leaf for VStack {
    fn render(&mut self) -> RenderResult {
        RenderResult::ToDo
    }
}
