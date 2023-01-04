use leafui::{
    nodes::{Label, Stack},
    Leaf, Shrub,
};
use taffy::style::{FlexDirection, Style};

fn main() {
    let mut shrub = Shrub::new();

    let main = content(&mut shrub);

    shrub.run(main);
}

fn content(shrub: &mut Shrub) -> Leaf {
    let text = shrub.new_leaf(Label::new("Hello, World!"), Style::default(), &[]);
    let second_text = shrub.new_leaf(Label::new("And hello again :)"), Style::default(), &[]);

    shrub.new_leaf(
        Stack::new(),
        Style {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        &[text, second_text],
    )
}
