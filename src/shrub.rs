use druid_shell::{Application, WindowBuilder};
use slotmap::SlotMap;

use crate::{window::Window, Leaf};

pub type LeafID = slotmap::DefaultKey;

#[derive(Debug)]
pub struct Shrub {
    leaves: SlotMap<LeafID, Box<dyn Leaf>>,
}

impl Shrub {
    pub fn new() -> Self {
        Self {
            leaves: SlotMap::with_capacity_and_key(1),
        }
    }

    pub fn run(mut self, leaf: impl Leaf) {
        leaf.design(&mut self);
        let leaf = leaf.create(&mut self);

        println!("{:#?}", self.leaves);

        let app = Application::new().unwrap();
        let mut builder = WindowBuilder::new(app.clone());
        builder.set_handler(Box::<Window>::default());
        builder.set_title("Hello example");

        let window = builder.build().unwrap();
        window.show();

        app.run(None);
    }

    #[inline]
    pub fn register_leaf(&mut self, leaf: Box<dyn Leaf>) -> LeafID {
        self.leaves.insert(leaf)
    }
}
