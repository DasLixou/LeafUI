use crate::{Blossom, Leaf};

#[derive(Debug)]
pub struct Stack;

impl Stack {
    pub const fn new() -> Self {
        Stack
    }
}

impl Blossom for Stack {
    fn render(&self, _children: &[Leaf]) {
        println!("Render Stack");
    }
}
