use crate::Style;

use super::StyleAttribute;

pub enum Padding {
    All(i32),
    Horizontal(i32),
    Vertical(i32),
    Left(i32),
    Right(i32),
    Top(i32),
    Bottom(i32),
}

impl StyleAttribute for Padding {
    fn apply(&self, style: &mut Style) {
        match *self {
            Padding::All(i) => {
                style.padding.left = i;
                style.padding.right = i;
                style.padding.top = i;
                style.padding.bottom = i;
            }
            Padding::Horizontal(i) => {
                style.padding.left = i;
                style.padding.right = i;
            }
            Padding::Vertical(i) => {
                style.padding.top = i;
                style.padding.bottom = i;
            }
            Padding::Left(i) => {
                style.padding.left = i;
            }
            Padding::Right(i) => {
                style.padding.right = i;
            }
            Padding::Top(i) => {
                style.padding.top = i;
            }
            Padding::Bottom(i) => {
                style.padding.bottom = i;
            }
        }
    }
}
