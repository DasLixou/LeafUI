use druid_shell::{Application, WindowBuilder};
use slotmap::{SecondaryMap, SlotMap};
use taffy::{
    prelude::{AvailableSpace, Size},
    style::Style,
    Taffy,
};

use crate::{window::Window, Blossom};

pub type Leaf = slotmap::DefaultKey;

struct LeafData {
    blossom: Box<dyn Blossom>,
}

pub struct Shrub {
    leaves: SlotMap<Leaf, LeafData>,
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
        let leaf = self.leaves.insert(LeafData {
            blossom: Box::new(blossom),
        });
        let _ = self.layout.new_with_children(style, children);
        self.children.insert(leaf, Vec::from(children));
        leaf
    }

    pub fn run(mut self, leaf: Leaf) {
        self.layout
            .compute_layout(leaf, {
                Size {
                    width: AvailableSpace::Definite(1280.),
                    height: AvailableSpace::Definite(720.),
                }
            })
            .unwrap();

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
        let layout = self.layout.layout(leaf).unwrap();
        self.leaves[leaf].blossom.render(layout);

        let children = self.children[leaf].as_slice();
        for leaf in children {
            self.render(*leaf);
        }
    }
}
