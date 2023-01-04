use druid_shell::{Application, WindowBuilder};
use slotmap::{SecondaryMap, SlotMap};
use taffy::{style::Style, Taffy};

use crate::window::Window;

pub type Leaf = slotmap::DefaultKey;

pub struct Shrub {
    leaves: SlotMap<Leaf, ()>,
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

    pub fn new_leaf(&mut self, style: Style, children: &[Leaf]) -> Leaf {
        let leaf = self.leaves.insert(());
        let _ = self.layout.new_leaf(style);
        self.children.insert(leaf, Vec::from(children));
        leaf
    }

    pub fn run(self, _leaf: Leaf) {
        let app = Application::new().unwrap();
        let mut builder = WindowBuilder::new(app.clone());
        builder.set_handler(Box::<Window>::default());
        builder.set_title("Hello example");

        let window = builder.build().unwrap();
        window.show();

        app.run(None);
    }
}
