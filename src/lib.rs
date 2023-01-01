mod window;

use leaf_nodes::{Leaf, LeafID, Shrub};
use window::Window;

pub struct LeafUI {
    shrub: Shrub,
    main_leaf: LeafID,
    window: Window,
}

impl LeafUI {
    pub fn new(leaf: impl Leaf) -> Self {
        let mut shrub = Shrub::new();

        println!("{:?}", leaf.layout(&mut shrub));
        println!("{:#?}", shrub);

        let leaf = leaf.create(&mut shrub);

        LeafUI {
            shrub: shrub,
            main_leaf: leaf,
            window: Window::new(),
        }
    }

    pub fn run(self) {
        self.window.run();
    }
}
