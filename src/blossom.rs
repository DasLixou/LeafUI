use downcast_rs::{impl_downcast, Downcast};
use taffy::{prelude::Layout, style::Style};

pub trait Blossom: Downcast {
    fn style(&self) -> Style;
    fn render(&self, layout: &Layout);
}
impl_downcast!(Blossom);
