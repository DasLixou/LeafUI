mod window;

use leaf_nodes::{Leaf, LeafID};
use window::Window;

pub struct LeafUI {
    leaf: LeafID,
    window: Window,
}

impl LeafUI {
    pub fn new(leaf: impl Leaf) -> Self {
        LeafUI {
            leaf: leaf.id(),
            window: Window::new(),
        }
    }

    pub fn run(self) {
        self.window.run();
    }
}
