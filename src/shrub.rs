use druid_shell::{Application, WindowBuilder};
use slotmap::{SecondaryMap, SlotMap};
use taffy::{style::Style, Taffy};

use crate::{window::Window, Blossom};

pub type Leaf = slotmap::DefaultKey;

pub struct Shrub {
    leaves: SlotMap<Leaf, Box<dyn Blossom>>,
    children: SecondaryMap<Leaf, Vec<Leaf>>,
    layout: Taffy,
}

impl Shrub {
    pub fn new() -> Self {
        Self {
            leaves: SlotMap::with_capacity(1),
            children: SecondaryMap::with_capacity(1),
            layout: Taffy::new(),
        }
    }

    pub fn new_leaf(&mut self, blossom: impl Blossom, style: Style, children: &[Leaf]) -> Leaf {
        let leaf = self.leaves.insert(Box::new(blossom));
        let _ = self.layout.new_leaf(style);
        self.children.insert(leaf, Vec::from(children));
        leaf
    }

    pub fn run(self, leaf: Leaf) {
        self.render(leaf);

        let app = Application::new().unwrap();
        let mut builder = WindowBuilder::new(app.clone());
        builder.set_handler(Box::<Window>::default());
        builder.set_title("Hello example");

        let window = builder.build().unwrap();
        window.show();

        app.run(None);
    }

    pub fn render(&self, leaf: Leaf) {
        let children = self.children[leaf].as_slice();
        self.leaves[leaf].render(self, children);
    }
}
