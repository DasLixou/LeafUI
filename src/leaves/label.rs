use const_default::ConstDefault;

use crate::{
    taffy::{
        prelude::{Layout, Size},
        style::Style,
    },
    Blossom,
};

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

impl ConstDefault for Label {
    const DEFAULT: Label = Label {
        text: String::new(),
    };
}
