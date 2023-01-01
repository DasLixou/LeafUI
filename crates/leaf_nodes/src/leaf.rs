use std::fmt::Debug;

use crate::{LeafID, Shrub};

pub trait Leaf: Debug {
    fn new() -> Self
    where
        Self: Sized;

    fn layout(&self, shrub: &mut Shrub) -> LeafID;

    fn create(self, shrub: &mut Shrub) -> LeafID;
}
