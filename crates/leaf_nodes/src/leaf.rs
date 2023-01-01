use std::fmt::Debug;

use crate::{LeafID, Shrub};

pub trait Leaf: Debug {
    fn new(cx: &mut Shrub) -> Self
    where
        Self: Sized;

    fn layout(&self, cx: &mut Shrub) -> Option<Box<dyn Leaf>>;

    fn id(&self) -> LeafID;
}
