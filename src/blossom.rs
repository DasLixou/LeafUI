use crate::taffy::{prelude::Layout, style::Style};
use downcast_rs::{impl_downcast, Downcast};

pub trait Blossom: Downcast {
    fn style(&self) -> Style;
    fn render(&self, layout: &Layout);
}
impl_downcast!(Blossom);
