use leafui::{
    nodes::{Label, Padding, VStack},
    Leaf, LeafID, Shrub,
};
use taffy::{
    prelude::{Node, Rect},
    style::Style,
    Taffy,
};

fn main() {
    Shrub::new().run(Content);
}

#[derive(Debug)]
struct Content;

impl Leaf for Content {
    fn layout(&self, taffy: &mut Taffy) -> Node {
        taffy.new_leaf(Style::default()).unwrap()
    }

    fn design(&self, shrub: &mut Shrub) -> Option<LeafID> {
        Some(
            Padding::new(Rect::from_points(5., 0., 2., 2.))
                .set_child(
                    VStack::new()
                        .add_child(Label::new("Hello, World!").create(shrub))
                        .add_child(Label::new("And hello again :)").create(shrub))
                        .create(shrub),
                )
                .create(shrub),
        )
    }

    fn create(self, shrub: &mut Shrub) -> LeafID {
        shrub.register_leaf(Box::new(self))
    }
}
