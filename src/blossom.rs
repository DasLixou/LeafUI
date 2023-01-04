use downcast_rs::{impl_downcast, Downcast};
use taffy::prelude::Layout;

use crate::{Leaf, Shrub};

pub trait Blossom: Downcast {
    fn render(&self, shrub: &Shrub, layout: &Layout, children: &[Leaf]);
}
impl_downcast!(Blossom);
