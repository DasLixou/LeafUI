use downcast_rs::{impl_downcast, Downcast};

use crate::Leaf;

pub trait Blossom: Downcast {
    fn render(&self, children: &[Leaf]);
}
impl_downcast!(Blossom);
