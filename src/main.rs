use std::process::id;

use leaf_nodes::{
    nodes::{Label, Padding, VStack},
    Leaf, LeafID, Shrub,
};
use leafui::LeafUI;

fn main() {
    //let mut leaf = Content::new();
    //println!("{:#?}", leaf.layout());

    //LeafUI::new(leaf).run();
}

#[derive(Debug)]
struct Content(LeafID);

impl Leaf for Content {
    fn new(cx: &mut leaf_nodes::Shrub) -> Self
    where
        Self: Sized,
    {
        Self(cx.register_leaf())
    }

    fn layout(&self, cx: &mut Shrub) -> Option<Box<dyn Leaf>> {
        Some(Box::new(
            Padding::new(cx).padding(5, 0, 2, 2).add_child(
                VStack::new(cx)
                    .add_child(Label::new(cx).text("Hello, World!").id())
                    .add_child(Label::new(cx).text("And hello again :)").id())
                    .id(),
            ),
        ))
    }

    fn id(&self) -> LeafID {
        self.0
    }
}
