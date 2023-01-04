use leafui::{
    nodes::{Label, Stack},
    Leaf, Shrub,
};
use taffy::{
    prelude::Size,
    style::{FlexDirection, Style},
};

fn main() {
    let mut shrub = Shrub::new();

    let main = content(&mut shrub);

    shrub.run(main);
}

fn content(shrub: &mut Shrub) -> Leaf {
    let text_style = Style {
        size: Size::from_points(28., 88.),
        ..Default::default()
    };

    let text = shrub.new_leaf(Label::new("Hello, World!"), text_style, &[]);
    let second_text = shrub.new_leaf(Label::new("And hello again :)"), text_style, &[]);

    shrub.new_leaf(
        Stack::new(),
        Style {
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        &[text, second_text],
    )
}
