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
                style.padding = [i, i, i, i];
            }
            Padding::Horizontal(i) => {
                style.padding[0] = i;
                style.padding[2] = i;
            }
            Padding::Vertical(i) => {
                style.padding[1] = i;
                style.padding[3] = i;
            }
            Padding::Left(i) => {
                style.padding[0] = i;
            }
            Padding::Right(i) => {
                style.padding[1] = i;
            }
            Padding::Top(i) => {
                style.padding[2] = i;
            }
            Padding::Bottom(i) => {
                style.padding[3] = i;
            }
        }
    }
}
