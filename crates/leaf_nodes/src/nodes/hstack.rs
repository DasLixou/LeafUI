use crate::{Branch, Leaf};

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
    fn layout(&mut self) -> Option<Box<dyn Leaf>> {
        None
    }
}
