use leaf_nodes::{attributes::padding::Padding, nodes::Label, Style};

fn main() {
    let node = Label("Hello, World!".into());
    let style = Style::of(&[
        Box::new(Padding::All(2)),
        Box::new(Padding::Left(5)),
        Box::new(Padding::Right(0)),
    ]);
    println!("{:?}", style);
}
