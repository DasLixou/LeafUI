use downcast_rs::{impl_downcast, Downcast};
use taffy::prelude::Layout;

pub trait Blossom: Downcast {
    fn render(&self, layout: &Layout);
}
impl_downcast!(Blossom);
