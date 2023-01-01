use crate::{Branch, Leaf};

#[derive(Debug)]
pub struct HStack {
    children: Vec<Box<dyn Leaf>>,
}

impl HStack {
    pub const fn new() -> Self {
        HStack { children: vec![] }
    }

    pub fn children(mut self, branch: impl Branch) -> Self {
        self.children = branch.resolve();
        self
    }
}

impl Leaf for HStack {
    fn layout(&mut self) -> Option<Box<dyn Leaf>> {
        None
    }
}
