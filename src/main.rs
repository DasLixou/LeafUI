use leaf_nodes::{
    nodes::{Label, Padding, VStack},
    Leaf, LeafID, Shrub,
};
use leafui::LeafUI;

fn main() {
    LeafUI::new(Content).run();
}

#[derive(Debug)]
struct Content;

impl Leaf for Content {
    fn layout(&self, shrub: &mut Shrub) -> LeafID {
        Padding::new(5, 0, 2, 2)
            .add_child(
                VStack::new()
                    .add_child(Label::new("Hello, World!").create(shrub))
                    .add_child(Label::new("And hello again :)").create(shrub))
                    .create(shrub),
            )
            .create(shrub)
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
