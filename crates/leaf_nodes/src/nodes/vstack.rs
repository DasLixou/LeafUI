use crate::{Branch, Leaf};

#[derive(Debug)]
pub struct VStack {
    children: Vec<Box<dyn Leaf>>,
}

impl VStack {
    pub const fn new() -> Self {
        VStack { children: vec![] }
    }

    pub fn children(mut self, branch: impl Branch) -> Self {
        self.children = branch.resolve();
        self
    }
}

impl Leaf for VStack {
    fn layout(&mut self) -> Option<Box<dyn Leaf>> {
        None
    }
}
