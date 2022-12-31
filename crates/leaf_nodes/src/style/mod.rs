use self::{attributes::StyleAttribute, data::padding::Padding};

pub mod attributes;
pub mod data;

#[derive(Default, Debug)]
pub struct Style {
    pub padding: Padding,
}

impl Style {
    pub fn with(mut self, attrib: impl StyleAttribute) -> Self {
        attrib.apply(&mut self);
        self
    }
}
