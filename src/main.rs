use leafui::{
    nodes::{Label, Stack},
    Leaf, Shrub,
};
use taffy::{
    prelude::Size,
    style::{AlignItems, FlexDirection, Style},
};

fn main() {
    let mut shrub = Shrub::new();

    let main = content(&mut shrub);

    shrub.run(main);
}

fn content(shrub: &mut Shrub) -> Leaf {
    let text = shrub.new_leaf(
        Label::new("Hello, World!"),
        Style {
            size: Size::from_points(60., 28.),
            ..Default::default()
        },
        &[],
    );
    let second_text = shrub.new_leaf(
        Label::new("And hello again :)"),
        Style {
            size: Size::from_points(88., 28.),
            ..Default::default()
        },
        &[],
    );

    shrub.new_leaf(
        Stack::new(),
        Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        &[text, second_text],
    )
}
