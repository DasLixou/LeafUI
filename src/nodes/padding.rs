use taffy::{prelude::Rect, style::Dimension};

#[derive(Debug)]
pub struct Padding {
    padding: Rect<Dimension>,
}

impl Padding {
    pub const fn new(padding: Rect<Dimension>) -> Self {
        Padding { padding }
    }

    pub const fn padding(mut self, padding: Rect<Dimension>) -> Self {
        self.padding = padding;
        self
    }
}
