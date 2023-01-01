use std::fmt::Debug;

use downcast_rs::{impl_downcast, Downcast};

pub trait Leaf: Downcast + Debug {
    fn layout(&mut self) -> Option<Box<dyn Leaf>>;
}
impl_downcast!(Leaf);
