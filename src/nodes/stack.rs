use taffy::{
    prelude::Layout,
    style::{AlignItems, FlexDirection, Style},
};

use crate::Blossom;

#[derive(Debug)]
pub struct Stack {
    pub flex_direction: FlexDirection,
    pub align_items: AlignItems,
}

impl Blossom for Stack {
    fn style(&self) -> Style {
        Style {
            flex_direction: self.flex_direction,
            align_items: self.align_items,
            ..Default::default()
        }
    }

    fn render(&self, layout: &Layout) {
        println!("Render Stack: {layout:?}");
    }
}
