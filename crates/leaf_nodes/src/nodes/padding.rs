use crate::{Branch, Leaf};

#[derive(Debug)]
pub struct Padding {
    children: Vec<Box<dyn Leaf>>,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Padding {
    pub const fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        Padding {
            children: vec![],
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn children(mut self, branch: impl Branch) -> Self {
        self.children = branch.resolve();
        self
    }
}

impl Leaf for Padding {
    fn layout(&mut self) -> Option<Box<dyn Leaf>> {
        None
    }
}
