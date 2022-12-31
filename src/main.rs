use leaf_nodes::{
    attributes::padding::Padding,
    nodes::{HStack, Label},
    Style,
};

fn main() {
    let style = Style::default()
        .with(Padding::All(2))
        .with(Padding::Left(5))
        .with(Padding::Right(0));
    let node = HStack::new().style(style).children((
        Label::new("Hello, World!"),
        Label::new("And hello again :)"),
    ));
    println!("{:#?}", node);
}
