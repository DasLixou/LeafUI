use crate::Leaf;

pub trait Branch {}

impl Branch for () {}

impl<D: Leaf> Branch for D {}

macro_rules! impl_branch {
    ( $(($generic:ident, $index:tt))+ ) => {
        impl<$($generic: Leaf),+> Branch for ($($generic,)+) {

        }
    };
}

impl_branch!((A, 0));
impl_branch!((A, 0)(B, 1));
impl_branch!((A, 0)(B, 1)(C, 2));
impl_branch!((A, 0)(B, 1)(C, 2)(D, 3));
impl_branch!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4));
impl_branch!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5));
impl_branch!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6));
impl_branch!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7));
impl_branch!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8));
impl_branch!((A, 0)(B, 1)(C, 2)(D, 3)(E, 4)(F, 5)(G, 6)(H, 7)(I, 8)(J, 9));
