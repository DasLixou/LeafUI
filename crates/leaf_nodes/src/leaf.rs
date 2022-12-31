use std::fmt::Debug;

use downcast_rs::{impl_downcast, Downcast};

#[derive(Debug)]
pub enum RenderResult {
    Component(Box<dyn Leaf>),
    ToDo,
}

impl<L: Leaf> From<L> for RenderResult {
    fn from(value: L) -> Self {
        RenderResult::Component(Box::new(value))
    }
}

pub trait Leaf: Downcast + Debug {
    fn render(&mut self) -> RenderResult;
}
impl_downcast!(Leaf);
