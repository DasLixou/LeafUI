use crate::Style;

pub trait StyleAttribute {
    fn apply(&self, style: &mut Style);
}

pub mod padding;
