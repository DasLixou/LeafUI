use std::fmt::Debug;

use downcast_rs::{impl_downcast, Downcast};

pub trait Leaf: Downcast + Debug {}
impl_downcast!(Leaf);
