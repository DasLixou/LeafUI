use crate::{Branch, Leaf};

#[derive(Debug)]
pub struct VStack(Vec<Box<dyn Leaf>>);

impl VStack {
    pub fn new() -> Self {
        VStack(vec![])
    }

    pub fn children(mut self, branch: impl Branch) -> Self {
        self.0 = branch.resolve();
        self
    }
}

impl Leaf for VStack {
    fn layout(&mut self) -> Option<Box<dyn Leaf>> {
        None
    }
}
