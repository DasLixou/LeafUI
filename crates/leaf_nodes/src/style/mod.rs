use self::{attributes::StyleAttribute, data::padding::Padding};

pub mod attributes;
pub mod data;

#[derive(Default, Debug)]
pub struct Style {
    pub padding: Padding,
}

impl Style {
    pub fn of(style_attribs: &[Box<dyn StyleAttribute>]) -> Self {
        let mut style = Self::default();
        style_attribs
            .iter()
            .for_each(|attrib| attrib.apply(&mut style));
        style
    }
}
