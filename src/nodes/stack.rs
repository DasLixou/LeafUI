use taffy::prelude::Layout;

use crate::{Blossom, Leaf, Shrub};

#[derive(Debug)]
pub struct Stack;

impl Stack {
    pub const fn new() -> Self {
        Stack
    }
}

impl Blossom for Stack {
    fn render(&self, shrub: &Shrub, layout: &Layout, children: &[Leaf]) {
        println!("Render Stack: {layout:?}");
        for leaf in children {
            shrub.render(*leaf);
        }
    }
}
