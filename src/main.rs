use leaf_nodes::{
    nodes::{Label, Padding, VStack},
    Leaf, RenderResult,
};
use leafui::LeafUI;

fn main() {
    let mut leaf = Content::new();
    println!("{:#?}", leaf.render());

    LeafUI::new(leaf).run();
}

#[derive(Debug)]
struct Content;

impl Content {
    pub fn new() -> Self {
        Self
    }
}

impl Leaf for Content {
    fn render(&mut self) -> RenderResult {
        Padding::new(5, 0, 2, 2)
            .children(VStack::new().children((
                Label::new("Hello, World!"),
                Label::new("And hello again :)"),
            )))
            .into()
    }
}
