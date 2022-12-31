use leaf_nodes::{
    attributes::padding::Padding,
    nodes::{Label, VStack},
    Leaf, Style,
};
use leafui::LeafUI;

fn main() {
    let leaf = app();
    println!("{:#?}", leaf);

    LeafUI::new(leaf).run();
}

fn app() -> impl Leaf {
    let style = Style::default()
        .with(Padding::All(2))
        .with(Padding::Left(5))
        .with(Padding::Right(0));

    VStack::new().style(style).children((
        Label::new("Hello, World!"),
        Label::new("And hello again :)"),
    ))
}
