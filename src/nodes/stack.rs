use taffy::{
    prelude::{Layout, Rect},
    style::{AlignItems, Dimension, FlexDirection, Style},
};

use crate::Blossom;

#[derive(Debug)]
pub struct Stack {
    pub flex_direction: FlexDirection,
    pub align_items: AlignItems,
    pub padding: Rect<Dimension>,
}

impl Blossom for Stack {
    fn style(&self) -> Style {
        Style {
            flex_direction: self.flex_direction,
            align_items: self.align_items,
            padding: self.padding,
            ..Default::default()
        }
    }

    fn render(&self, layout: &Layout) {
        println!("Render Stack: {layout:?}");
    }
}

impl Stack {
    pub const DEFAULT: Stack = Stack {
        flex_direction: Style::DEFAULT.flex_direction,
        align_items: Style::DEFAULT.align_items,
        padding: Style::DEFAULT.padding,
    };
}

impl Default for Stack {
    fn default() -> Self {
        Self::DEFAULT
    }
}
