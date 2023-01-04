use downcast_rs::{impl_downcast, Downcast};
use druid_shell::piet::Piet;
use taffy::{prelude::Layout, style::Style};

pub trait Blossom: Downcast {
    fn style(&self) -> Style;
    fn render(&self, layout: &Layout, piet: &mut Piet);
}
impl_downcast!(Blossom);
