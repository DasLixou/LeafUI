use leaf_nodes::{attributes::padding::Padding, nodes::Label, Style};

fn main() {
    let node = Label("Hello, World!".into());
    let style = Style::default()
        .with(Padding::All(2))
        .with(Padding::Left(5))
        .with(Padding::Right(0));
    println!("{:?}", style);
}
