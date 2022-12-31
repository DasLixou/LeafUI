use self::attributes::StyleAttribute;

pub mod attributes;

#[derive(Default, Debug)]
pub struct Style {
    pub padding: [i32; 4],
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
