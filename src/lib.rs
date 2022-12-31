mod window;

use leaf_nodes::Leaf;
use window::Window;

pub struct LeafUI {
    leaf: Box<dyn Leaf>,
    window: Window,
}

impl LeafUI {
    pub fn new(leaf: impl Leaf) -> Self {
        LeafUI {
            leaf: Box::new(leaf),
            window: Window::new(),
        }
    }

    pub fn run(self) {
        self.window.run();
    }
}
