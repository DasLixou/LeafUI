use crate::{Branch, Leaf, RenderResult};

#[derive(Debug)]
pub struct HStack(Vec<Box<dyn Leaf>>);

impl HStack {
    pub fn new() -> Self {
        HStack(vec![])
    }

    pub fn children(mut self, branch: impl Branch) -> Self {
        self.0 = branch.resolve();
        self
    }
}

impl Leaf for HStack {
    fn render(&mut self) -> RenderResult {
        RenderResult::ToDo
    }
}
