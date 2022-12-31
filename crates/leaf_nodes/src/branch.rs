use crate::Leaf;

pub trait Branch {
    fn resolve(self) -> Vec<Box<dyn Leaf>>;
}

impl Branch for () {
    fn resolve(self) -> Vec<Box<dyn Leaf>> {
        vec![]
    }
}

impl<D: Leaf> Branch for D {
    fn resolve(self) -> Vec<Box<dyn Leaf>> {
        vec![Box::new(self)]
    }
}

macro_rules! impl_branch {
    ( $(($generic:ident, $index:tt))+ ) => {
        impl<$($generic: Leaf),+> Branch for ($($generic,)+) {
            fn resolve(self) -> Vec<Box<dyn Leaf>> {
                let mut branch = Vec::<Box<dyn Leaf>>::new();
                $(
                    branch.push(Box::new(self.$index));
                )+
                branch
            }
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
