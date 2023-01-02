use std::fmt::Debug;

use taffy::{prelude::Node, Taffy};

use crate::{LeafID, Shrub};

pub trait Leaf: Debug {
    fn layout(&self, taffy: &mut Taffy) -> Node;

    fn design(&self, shrub: &mut Shrub) -> Option<LeafID>;

    fn create(self, shrub: &mut Shrub) -> LeafID;
}
