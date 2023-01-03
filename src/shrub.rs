use druid_shell::{Application, WindowBuilder};
use slotmap::{SecondaryMap, SlotMap};

use crate::window::Window;

pub type Leaf = slotmap::DefaultKey;

#[derive(Debug)]
pub struct Shrub {
    leaves: SlotMap<Leaf, ()>,
    children: SecondaryMap<Leaf, Vec<Leaf>>,
}

impl Shrub {
    pub fn new() -> Self {
        Self {
            leaves: SlotMap::with_capacity(1),
            children: SecondaryMap::with_capacity(1),
        }
    }

    pub fn new_leaf(&mut self, children: &[Leaf]) -> Leaf {
        let leaf = self.leaves.insert(());
        self.children.insert(leaf, Vec::from(children));
        leaf
    }

    pub fn run(self, _leaf: Leaf) {
        println!("{:#?}", self);

        let app = Application::new().unwrap();
        let mut builder = WindowBuilder::new(app.clone());
        builder.set_handler(Box::<Window>::default());
        builder.set_title("Hello example");

        let window = builder.build().unwrap();
        window.show();

        app.run(None);
    }
}
