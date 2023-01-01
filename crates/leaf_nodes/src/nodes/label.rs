use crate::{Leaf, RenderResult};

#[derive(Debug)]
pub struct Label(String);

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label(text.into())
    }
}

impl Leaf for Label {
    fn render(&mut self) -> RenderResult {
        RenderResult::ToDo
    }
}
