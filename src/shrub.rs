use druid_shell::{Application, WindowBuilder};
use hashbrown::HashMap;

use crate::{window::Window, Leaf, LeafID};

#[derive(Debug)]
pub struct Shrub {
    counter: u64,
    leaves: HashMap<LeafID, Box<dyn Leaf>>,
    main_leaf: LeafID,
}

impl Shrub {
    pub fn new(main_leaf: impl Leaf) -> Self {
        let counter = 1;
        let id = LeafID(counter);
        let mut leaves = HashMap::new();
        leaves.insert(id, Box::new(main_leaf));

        Self {
            counter,
            leaves: HashMap::new(),
            main_leaf: id,
        }
    }

    pub fn run(self) {
        let app = Application::new().unwrap();
        let mut builder = WindowBuilder::new(app.clone());
        builder.set_handler(Box::<Window>::default());
        builder.set_title("Hello example");

        let window = builder.build().unwrap();
        window.show();

        app.run(None);
    }

    pub fn register_leaf(&mut self, leaf: Box<dyn Leaf>) -> LeafID {
        self.counter += 1;
        let id = LeafID(self.counter);
        self.leaves.insert(id, leaf);
        id
    }
}
