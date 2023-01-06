use taffy::{
    prelude::{Layout, Size},
    style::Style,
};

use crate::Blossom;

#[derive(Debug)]
pub struct Label {
    pub text: String,
}

impl Blossom for Label {
    fn style(&self) -> Style {
        Style {
            size: Size::from_points(88., 28.),
            ..Default::default()
        }
    }

    fn render(&self, layout: &Layout) {
        println!("Render Label: {layout:?}");
    }
}

impl Label {
    pub const DEFAULT: Label = Label {
        text: String::new(),
    };
}

impl Default for Label {
    fn default() -> Self {
        Self::DEFAULT
    }
}
