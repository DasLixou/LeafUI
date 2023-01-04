use taffy::prelude::Layout;

use crate::Blossom;

#[derive(Debug)]
pub struct Stack;

impl Stack {
    pub const fn new() -> Self {
        Stack
    }
}

impl Blossom for Stack {
    fn render(&self, layout: &Layout) {
        println!("Render Stack: {layout:?}");
    }
}
