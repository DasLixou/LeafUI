use downcast_rs::{impl_downcast, Downcast};

use crate::{Leaf, Shrub};

pub trait Blossom: Downcast {
    fn render(&self, shrub: &Shrub, children: &[Leaf]);
}
impl_downcast!(Blossom);
