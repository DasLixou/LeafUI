use leaf_nodes::{
    attributes::padding::Padding,
    nodes::{Label, VStack},
    Leaf, RenderResult, Style,
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
        VStack::new()
            .style(
                Style::default()
                    .with(Padding::All(2))
                    .with(Padding::Left(5))
                    .with(Padding::Right(0)),
            )
            .children((
                Label::new("Hello, World!"),
                Label::new("And hello again :)"),
            ))
            .into()
    }
}
