mod window;

mod leaf;
use druid_shell::{Application, WindowBuilder};
pub use leaf::*;

mod shrub;
pub use shrub::*;

pub mod nodes;

use window::Window;

pub struct LeafUI {
    shrub: Shrub,
    main_leaf: LeafID,
}

impl LeafUI {
    pub fn new(leaf: impl Leaf) -> Self {
        let mut shrub = Shrub::new();

        println!("{:?}", leaf.design(&mut shrub));
        println!("{:#?}", shrub);

        let leaf = leaf.create(&mut shrub);

        LeafUI {
            shrub: shrub,
            main_leaf: leaf,
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
}
